use std::sync::Arc;

use axum::{
	extract::{Path, State, WebSocketUpgrade},
	http::StatusCode,
	response::{sse::Event, IntoResponse, Sse},
	Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
	engine::{GenOptions, InferenceEngine},
	model_registry::ModelSpec,
	server::AppState,
	templates::{detect_template_family, TemplateFamily},
};

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
	pub model: String,
	pub prompt: Option<String>,
	pub messages: Option<Vec<ChatMessage>>,
	pub system: Option<String>,
	pub max_tokens: Option<usize>,
	pub temperature: Option<f32>,
	pub top_p: Option<f32>,
	pub top_k: Option<i32>,
	pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
	pub role: String,
	pub content: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
	pub response: String,
}

#[derive(Debug, Serialize)]
pub struct ModelListResponse {
	pub models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize)]
pub struct ModelInfo {
	pub name: String,
	pub path: String,
	pub size_mb: f64,
	pub model_type: String,
	pub quantization: Option<String>,
	pub is_discovered: bool,
}

#[derive(Debug)]
enum ApiError {
	ModelNotFound(String),
	GenerationFailed(String),
	InvalidRequest(String),
}

impl ApiError {
	fn status(&self) -> StatusCode {
		match self {
			ApiError::ModelNotFound(_) => StatusCode::NOT_FOUND,
			ApiError::GenerationFailed(_) => StatusCode::BAD_GATEWAY,
			ApiError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
		}
	}

	fn message(&self) -> String {
		match self {
			ApiError::ModelNotFound(m) => m.clone(),
			ApiError::GenerationFailed(m) => m.clone(),
			ApiError::InvalidRequest(m) => m.clone(),
		}
	}
}

impl IntoResponse for ApiError {
	fn into_response(self) -> axum::response::Response {
		(self.status(), Json(json!({ "error": self.message() }))).into_response()
	}
}

pub async fn generate(State(state): State<Arc<AppState>>, Json(req): Json<GenerateRequest>) -> impl IntoResponse {
	match generate_inner(state, req).await {
		Ok(resp) => resp,
		Err(e) => e.into_response(),
	}
}

async fn generate_inner(state: Arc<AppState>, req: GenerateRequest) -> Result<axum::response::Response, ApiError> {
	let spec = {
		let reg = state.registry.read().await;
		reg.to_spec(&req.model)
			.ok_or_else(|| ApiError::ModelNotFound(format!("Model not found: {}", req.model)))?
	};

	let loaded = state
		.engine
		.load(&spec)
		.await
		.map_err(|e| ApiError::GenerationFailed(e.to_string()))?;

	let (prompt, family) = build_prompt(&spec, &req)?;

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
	if let Some(v) = req.top_k {
		opts.top_k = v;
	}
	let stream = req.stream.unwrap_or(false);
	opts.stream = stream;
	opts.stop_tokens = family.stop_tokens();

	if stream {
		let (tx, rx) = mpsc::unbounded_channel::<Result<Event, std::convert::Infallible>>();
		let prompt_clone = prompt.clone();
		let tx_tokens = tx.clone();
		tokio::spawn(async move {
			let on_token = move |tok: String| {
				let _ = tx_tokens.send(Ok(Event::default().data(tok)));
			};

			let _ = loaded
				.generate(&prompt_clone, opts, Some(Box::new(on_token)))
				.await;
			let _ = tx.send(Ok(Event::default().data("[DONE]")));
		});

		let stream = UnboundedReceiverStream::new(rx);
		let sse = Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default());
		return Ok(sse.into_response());
	}

	let response = loaded
		.generate(&prompt, opts, None)
		.await
		.map_err(|e| ApiError::GenerationFailed(e.to_string()))?;

	Ok(Json(GenerateResponse { response }).into_response())
}

fn build_prompt(spec: &ModelSpec, req: &GenerateRequest) -> Result<(String, TemplateFamily), ApiError> {
	if let Some(p) = &req.prompt {
		let fam = template_from_spec(spec);
		return Ok((p.clone(), fam));
	}

	let messages = req
		.messages
		.as_ref()
		.ok_or_else(|| ApiError::InvalidRequest("Either prompt or messages must be provided".into()))?;

	let (system, history, last_user) = split_messages(req.system.as_deref(), messages);
	let fam = template_from_spec(spec);
	let prompt = fam.render(system.as_deref(), &history, last_user.as_deref());
	Ok((prompt, fam))
}

fn template_from_spec(spec: &ModelSpec) -> TemplateFamily {
	match spec.template.as_deref() {
		Some("chatml") => TemplateFamily::ChatML,
		Some("llama3") | Some("llama-3") => TemplateFamily::Llama3,
		_ => detect_template_family(&spec.name),
	}
}

pub(crate) fn split_messages(
	system_override: Option<&str>,
	messages: &[ChatMessage],
) -> (Option<String>, Vec<(String, String)>, Option<String>) {
	let mut system = system_override.map(|s| s.to_string());
	if system.is_none() {
		if let Some(m) = messages.iter().find(|m| m.role == "system") {
			system = Some(m.content.clone());
		}
	}

	let mut history: Vec<(String, String)> = vec![];
	let mut pending_user: Option<String> = None;
	let mut last_user: Option<String> = None;

	for (idx, m) in messages.iter().enumerate() {
		match m.role.as_str() {
			"system" => {}
			"user" => {
				if idx == messages.len() - 1 {
					last_user = Some(m.content.clone());
				} else {
					pending_user = Some(m.content.clone());
				}
			}
			"assistant" => {
				if let Some(u) = pending_user.take() {
					history.push((u, m.content.clone()));
				}
			}
			_ => {}
		}
	}

	(system, history, last_user)
}

pub async fn list_models(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let mut models: Vec<ModelInfo> = vec![];
	let reg = state.registry.read().await;

	for entry in reg.inner.values() {
		let size_mb = std::fs::metadata(&entry.base_path)
			.map(|m| (m.len() as f64) / (1024.0 * 1024.0))
			.unwrap_or(0.0);
		let model_type = entry
			.base_path
			.extension()
			.and_then(|s| s.to_str())
			.unwrap_or("")
			.to_string();
		models.push(ModelInfo {
			name: entry.name.clone(),
			path: entry.base_path.to_string_lossy().to_string(),
			size_mb,
			model_type,
			quantization: None,
			is_discovered: false,
		});
	}

	for discovered in reg.discovered_models.values() {
		let size_mb = (discovered.size_bytes as f64) / (1024.0 * 1024.0);
		models.push(ModelInfo {
			name: discovered.name.clone(),
			path: discovered.path.to_string_lossy().to_string(),
			size_mb,
			model_type: discovered.model_type.clone(),
			quantization: discovered.quantization.clone(),
			is_discovered: true,
		});
	}

	Json(ModelListResponse { models })
}

pub async fn discover_models(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	{
		let mut reg = state.registry.write().await;
		reg.refresh_discovered_models();
	}
	list_models(State(state)).await
}

pub async fn load_model(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> impl IntoResponse {
	let spec = {
		let reg = state.registry.read().await;
		match reg.to_spec(&name) {
			Some(s) => s,
			None => return ApiError::ModelNotFound(format!("Model not found: {}", name)).into_response(),
		}
	};
	match state.engine.load(&spec).await {
		Ok(_) => Json(json!({"ok": true})).into_response(),
		Err(e) => ApiError::GenerationFailed(e.to_string()).into_response(),
	}
}

pub async fn unload_model(_state: State<Arc<AppState>>, Path(name): Path<String>) -> impl IntoResponse {
	Json(json!({"model": name, "unloaded": false}))
}

pub async fn model_status(_state: State<Arc<AppState>>, Path(name): Path<String>) -> impl IntoResponse {
	Json(json!({"model": name, "status": "unknown", "loaded": false}))
}

pub async fn ws_generate(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
	ws.on_upgrade(move |socket| handle_ws_generate(state, socket))
}

#[derive(Debug, Deserialize)]
struct WsGenerateRequest {
	model: String,
	prompt: String,
	max_tokens: Option<usize>,
	temperature: Option<f32>,
	top_p: Option<f32>,
	top_k: Option<i32>,
}

async fn handle_ws_generate(state: Arc<AppState>, mut socket: axum::extract::ws::WebSocket) {
	use axum::extract::ws::Message;

	let Some(Ok(Message::Text(text))) = socket.recv().await else {
		let _ = socket.send(Message::Close(None)).await;
		return;
	};

	let req: WsGenerateRequest = match serde_json::from_str(&text) {
		Ok(r) => r,
		Err(_) => {
			let _ = socket
				.send(Message::Text("{\"error\":\"invalid_request\"}".into()))
				.await;
			let _ = socket.send(Message::Close(None)).await;
			return;
		}
	};

	let spec = {
		let reg = state.registry.read().await;
		reg.to_spec(&req.model)
	};
	let Some(spec) = spec else {
		let _ = socket
			.send(Message::Text(format!("{{\"error\":\"model_not_found\",\"model\":\"{}\"}}", req.model)))
			.await;
		let _ = socket.send(Message::Close(None)).await;
		return;
	};

	let loaded = match state.engine.load(&spec).await {
		Ok(m) => m,
		Err(e) => {
			let _ = socket
				.send(Message::Text(format!("{{\"error\":\"load_failed\",\"message\":\"{}\"}}", e)))
				.await;
			let _ = socket.send(Message::Close(None)).await;
			return;
		}
	};

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
	if let Some(v) = req.top_k {
		opts.top_k = v;
	}

	let (tx, mut rx) = mpsc::unbounded_channel::<String>();
	let prompt = req.prompt.clone();
	let tx_tokens = tx.clone();
	let gen = tokio::spawn(async move {
		let cb = move |t: String| {
			let _ = tx_tokens.send(t);
		};
		let _ = loaded.generate(&prompt, opts, Some(Box::new(cb))).await;
		let _ = tx.send("[DONE]".to_string());
	});

	while let Some(tok) = rx.recv().await {
		let _ = socket.send(Message::Text(tok)).await;
	}

	let _ = gen.await;
	let _ = socket.send(Message::Text("{\"done\": true}".into())).await;
	let _ = socket.send(Message::Close(None)).await;
}

pub async fn list_tools() -> impl IntoResponse {
	Json(json!({"tools": Vec::<Value>::new()}))
}

pub async fn execute_tool(Path(name): Path<String>) -> impl IntoResponse {
	(
		StatusCode::NOT_IMPLEMENTED,
		Json(json!({"error": format!("tool not implemented: {}", name)})),
	)
}

pub async fn execute_workflow() -> impl IntoResponse {
	(
		StatusCode::NOT_IMPLEMENTED,
		Json(json!({"error": "workflow execution not implemented"})),
	)
}
