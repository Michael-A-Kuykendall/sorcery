use std::sync::Arc;

use axum::{
	extract::State,
	http::StatusCode,
	response::{sse::Event, IntoResponse, Sse},
	Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

use crate::{
	engine::GenOptions,
	server::AppState,
	templates::{detect_template_family, TemplateFamily},
};

#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
	pub model: String,
	pub messages: Vec<crate::api::ChatMessage>,
	pub stream: Option<bool>,
	pub temperature: Option<f32>,
	pub max_tokens: Option<usize>,
	pub top_p: Option<f32>,
	pub stop: Option<StopTokens>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StopTokens {
	Single(String),
	Multiple(Vec<String>),
}

impl StopTokens {
	pub fn into_vec(self) -> Vec<String> {
		match self {
			StopTokens::Single(s) => vec![s],
			StopTokens::Multiple(v) => v,
		}
	}
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionResponse {
	pub id: String,
	pub object: String,
	pub created: u64,
	pub model: String,
	pub choices: Vec<Choice>,
	pub usage: Usage,
}

#[derive(Debug, Serialize)]
pub struct Choice {
	pub index: usize,
	pub message: crate::api::ChatMessage,
	pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Usage {
	pub prompt_tokens: usize,
	pub completion_tokens: usize,
	pub total_tokens: usize,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
	pub id: String,
	pub object: String,
	pub created: u64,
	pub model: String,
	pub choices: Vec<ChunkChoice>,
}

#[derive(Debug, Serialize)]
pub struct ChunkChoice {
	pub index: usize,
	pub delta: Delta,
	pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Delta {
	pub role: Option<String>,
	pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
	pub object: String,
	pub data: Vec<ListModel>,
}

#[derive(Debug, Serialize)]
pub struct ListModel {
	pub id: String,
	pub object: String,
	pub created: u64,
	pub owned_by: String,
}

pub async fn chat_completions(State(state): State<Arc<AppState>>, Json(req): Json<ChatCompletionRequest>) -> impl IntoResponse {
	let spec = {
		let reg = state.registry.read().await;
		match reg.to_spec(&req.model) {
			Some(s) => s,
			None => {
				return (
					StatusCode::NOT_FOUND,
					Json(json!({
						"error": {
							"message": format!("Model not found: {}", req.model),
							"type": "invalid_request_error",
							"code": "model_not_found"
						}
					})),
				)
					.into_response();
			}
		}
	};

	let loaded = match state.engine.load(&spec).await {
		Ok(m) => m,
		Err(e) => {
			return (
				StatusCode::BAD_GATEWAY,
				Json(json!({"error": {"message": e.to_string(), "type": "server_error"}})),
			)
				.into_response();
		}
	};

	let fam = match spec.template.as_deref() {
		Some("chatml") => TemplateFamily::ChatML,
		Some("llama3") | Some("llama-3") => TemplateFamily::Llama3,
		_ => detect_template_family(&req.model),
	};

	let (system, history, last_user) = crate::api::split_messages(None, &req.messages);
	let prompt = fam.render(system.as_deref(), &history, last_user.as_deref());

	let mut opts = GenOptions::default();
	if let Some(v) = req.max_tokens {
		opts.max_tokens = v;
	}
	if let Some(v) = req.temperature {
		opts.temperature = v;
	}
	if let Some(v) = req.top_p {
		opts.top_p = v;
	}
	let stream = req.stream.unwrap_or(false);
	opts.stream = stream;

	let mut stops = fam.stop_tokens();
	if let Some(stop) = req.stop {
		stops.extend(stop.into_vec());
	}
	opts.stop_tokens = stops;

	let id = format!("chatcmpl-{}", Uuid::new_v4());
	let created = chrono::Utc::now().timestamp() as u64;

	if stream {
		let (tx, rx) = mpsc::unbounded_channel::<Result<Event, std::convert::Infallible>>();
		let prompt_clone = prompt.clone();
		let model_name = req.model.clone();
		let id_clone = id.clone();
		let tx_tokens = tx.clone();
		let id_for_tokens = id_clone.clone();
		let model_for_tokens = model_name.clone();

		tokio::spawn(async move {
			let init = ChatCompletionChunk {
				id: id_clone.clone(),
				object: "chat.completion.chunk".into(),
				created,
				model: model_name.clone(),
				choices: vec![ChunkChoice {
					index: 0,
					delta: Delta {
						role: Some("assistant".into()),
						content: None,
					},
					finish_reason: None,
				}],
			};
			let _ = tx.send(Ok(Event::default().data(serde_json::to_string(&init).unwrap())));

			let on_token = move |tok: String| {
				let chunk = ChatCompletionChunk {
					id: id_for_tokens.clone(),
					object: "chat.completion.chunk".into(),
					created,
					model: model_for_tokens.clone(),
					choices: vec![ChunkChoice {
						index: 0,
						delta: Delta {
							role: None,
							content: Some(tok),
						},
						finish_reason: None,
					}],
				};
				let _ = tx_tokens.send(Ok(Event::default().data(serde_json::to_string(&chunk).unwrap())));
			};

			let _ = loaded
				.generate(&prompt_clone, opts, Some(Box::new(on_token)))
				.await;

			let done_chunk = ChatCompletionChunk {
				id: id_clone.clone(),
				object: "chat.completion.chunk".into(),
				created,
				model: model_name.clone(),
				choices: vec![ChunkChoice {
					index: 0,
					delta: Delta { role: None, content: None },
					finish_reason: Some("stop".into()),
				}],
			};
			let _ = tx.send(Ok(Event::default().data(serde_json::to_string(&done_chunk).unwrap())));
			let _ = tx.send(Ok(Event::default().data("[DONE]")));
		});

		let stream = UnboundedReceiverStream::new(rx);
		return Sse::new(stream)
			.keep_alive(axum::response::sse::KeepAlive::default())
			.into_response();
	}

	let text = match loaded.generate(&prompt, opts, None).await {
		Ok(t) => t,
		Err(e) => {
			return (
				StatusCode::BAD_GATEWAY,
				Json(json!({"error": {"message": e.to_string(), "type": "server_error"}})),
			)
				.into_response();
		}
	};

	let resp = ChatCompletionResponse {
		id,
		object: "chat.completion".into(),
		created,
		model: req.model,
		choices: vec![Choice {
			index: 0,
			message: crate::api::ChatMessage {
				role: "assistant".into(),
				content: text,
			},
			finish_reason: Some("stop".into()),
		}],
		usage: Usage {
			prompt_tokens: 0,
			completion_tokens: 0,
			total_tokens: 0,
		},
	};

	Json(resp).into_response()
}

pub async fn models(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let now = chrono::Utc::now().timestamp() as u64;
	let reg = state.registry.read().await;
	let data = reg
		.list_all_available()
		.into_iter()
		.map(|id| ListModel {
			id,
			object: "model".into(),
			created: now,
			owned_by: "shimmy".into(),
		})
		.collect();

	Json(ModelsResponse {
		object: "list".into(),
		data,
	})
}
