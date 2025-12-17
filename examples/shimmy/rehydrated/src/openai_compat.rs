//! OpenAI Compatibility Layer
//! 
//! Rehydrated from: openai_compat.spell
//! 
//! Drop-in replacement for OpenAI API endpoints.

use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Sse},
    Json,
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, pin::Pin, sync::Arc, time::SystemTime};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::engine::{GenOptions, InferenceEngine};
use crate::server::AppState;
use crate::templates::detect_template_family;

// ═══════════════════════════════════════════════════════════════════
// OpenAI-Compatible Types
// From openai_compat.spell: @OpenAIChatRequest, @ChatCompletionResponse
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct OpenAIChatRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    #[serde(default)]
    pub frequency_penalty: Option<f32>,
}

fn default_max_tokens() -> Option<usize> { Some(256) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
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
    pub message: OpenAIMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

// Streaming types
#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Serialize)]
pub struct StreamChoice {
    pub index: usize,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Delta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════
// Model List
// From openai_compat.spell: GET /v1/models
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

/// GET /v1/models
/// OpenAI-compatible model listing
pub async fn list_models<E: InferenceEngine + Send + Sync>(
    State(state): State<Arc<AppState<E>>>,
) -> impl IntoResponse {
    let registry = state.registry.read().await;
    let models = registry.list_all_available();
    
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let data: Vec<ModelInfo> = models.into_iter()
        .map(|name| ModelInfo {
            id: name,
            object: "model".to_string(),
            created: now,
            owned_by: "local".to_string(),
        })
        .collect();
    
    Json(ModelsResponse {
        object: "list".to_string(),
        data,
    })
}

// ═══════════════════════════════════════════════════════════════════
// Chat Completions
// From openai_compat.spell: POST /v1/chat/completions
// ═══════════════════════════════════════════════════════════════════

/// POST /v1/chat/completions
/// Main OpenAI-compatible chat endpoint
pub async fn chat_completions<E: InferenceEngine + Send + Sync + 'static>(
    State(state): State<Arc<AppState<E>>>,
    Json(req): Json<OpenAIChatRequest>,
) -> impl IntoResponse {
    if req.stream {
        // SSE streaming response
        stream_chat_completions(state, req).await.into_response()
    } else {
        // Non-streaming response
        non_stream_chat_completions(state, req).await.into_response()
    }
}

async fn non_stream_chat_completions<E: InferenceEngine + Send + Sync>(
    state: Arc<AppState<E>>,
    req: OpenAIChatRequest,
) -> impl IntoResponse {
    let id = format!("chatcmpl-{}", Uuid::new_v4());
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Resolve model
    let spec = match state.registry.read().await.to_spec(&req.model) {
        Some(spec) => spec,
        None => {
            return Json(serde_json::json!({
                "error": {
                    "message": format!("Model '{}' not found", req.model),
                    "type": "invalid_request_error",
                    "code": "model_not_found"
                }
            })).into_response();
        }
    };
    
    // Format prompt
    let prompt = format_openai_messages(&req.model, &req.messages);
    let template = detect_template_family(&req.model);
    
    // Build options
    let options = GenOptions {
        max_tokens: req.max_tokens.unwrap_or(256),
        temperature: req.temperature.unwrap_or(0.7),
        top_p: req.top_p.unwrap_or(0.95),
        stop_sequences: req.stop.unwrap_or_else(|| template.stop_tokens()),
        ..Default::default()
    };
    
    // Load and generate
    let model = match state.engine.load(&spec).await {
        Ok(m) => m,
        Err(e) => {
            return Json(serde_json::json!({
                "error": {
                    "message": format!("Failed to load model: {}", e),
                    "type": "server_error"
                }
            })).into_response();
        }
    };
    
    match model.generate(&prompt, options).await {
        Ok(text) => {
            let response = ChatCompletionResponse {
                id,
                object: "chat.completion".to_string(),
                created: now,
                model: req.model,
                choices: vec![Choice {
                    index: 0,
                    message: OpenAIMessage {
                        role: "assistant".to_string(),
                        content: text.clone(),
                    },
                    finish_reason: "stop".to_string(),
                }],
                usage: Usage {
                    prompt_tokens: estimate_tokens(&prompt),
                    completion_tokens: estimate_tokens(&text),
                    total_tokens: estimate_tokens(&prompt) + estimate_tokens(&text),
                },
            };
            Json(response).into_response()
        }
        Err(e) => {
            Json(serde_json::json!({
                "error": {
                    "message": format!("Generation failed: {}", e),
                    "type": "server_error"
                }
            })).into_response()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// SSE Streaming
// From openai_compat.spell: server_sent_events with yield semantics
// ═══════════════════════════════════════════════════════════════════

async fn stream_chat_completions<E: InferenceEngine + Send + Sync + 'static>(
    state: Arc<AppState<E>>,
    req: OpenAIChatRequest,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let id = format!("chatcmpl-{}", Uuid::new_v4());
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let model_name = req.model.clone();
    
    // Channel for streaming tokens
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Spawn generation task
    let state_clone = state.clone();
    let req_clone = req;
    tokio::spawn(async move {
        // Resolve model
        let spec = match state_clone.registry.read().await.to_spec(&req_clone.model) {
            Some(spec) => spec,
            None => return,
        };
        
        // Format prompt
        let prompt = format_openai_messages(&req_clone.model, &req_clone.messages);
        let template = detect_template_family(&req_clone.model);
        
        let options = GenOptions {
            max_tokens: req_clone.max_tokens.unwrap_or(256),
            temperature: req_clone.temperature.unwrap_or(0.7),
            top_p: req_clone.top_p.unwrap_or(0.95),
            stop_sequences: req_clone.stop.unwrap_or_else(|| template.stop_tokens()),
            ..Default::default()
        };
        
        // Load model
        let model = match state_clone.engine.load(&spec).await {
            Ok(m) => m,
            Err(_) => return,
        };
        
        // Generate with callback
        let tx_clone = tx.clone();
        let callback = Box::new(move |token: &str| {
            let tx = tx_clone.clone();
            let token = token.to_string();
            tokio::spawn(async move {
                let _ = tx.send(token).await;
            });
        });
        
        let _ = model.generate_stream(&prompt, options, callback).await;
        // Signal end by dropping tx
    });
    
    // Build SSE stream
    let stream = async_stream::stream! {
        // Initial chunk with role
        let initial_chunk = ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk".to_string(),
            created: now,
            model: model_name.clone(),
            choices: vec![StreamChoice {
                index: 0,
                delta: Delta {
                    role: Some("assistant".to_string()),
                    content: None,
                },
                finish_reason: None,
            }],
        };
        let data = serde_json::to_string(&initial_chunk).unwrap();
        yield Ok(Event::default().data(data));
        
        // Stream tokens
        while let Some(token) = rx.recv().await {
            let chunk = ChatCompletionChunk {
                id: id.clone(),
                object: "chat.completion.chunk".to_string(),
                created: now,
                model: model_name.clone(),
                choices: vec![StreamChoice {
                    index: 0,
                    delta: Delta {
                        role: None,
                        content: Some(token),
                    },
                    finish_reason: None,
                }],
            };
            let data = serde_json::to_string(&chunk).unwrap();
            yield Ok(Event::default().data(data));
        }
        
        // Final chunk
        let final_chunk = ChatCompletionChunk {
            id: id.clone(),
            object: "chat.completion.chunk".to_string(),
            created: now,
            model: model_name.clone(),
            choices: vec![StreamChoice {
                index: 0,
                delta: Delta {
                    role: None,
                    content: None,
                },
                finish_reason: Some("stop".to_string()),
            }],
        };
        let data = serde_json::to_string(&final_chunk).unwrap();
        yield Ok(Event::default().data(data));
        
        // OpenAI-style terminator
        yield Ok(Event::default().data("[DONE]"));
    };
    
    Sse::new(stream)
}

// ═══════════════════════════════════════════════════════════════════
// Utilities
// ═══════════════════════════════════════════════════════════════════

/// Format OpenAI messages into model-specific prompt
fn format_openai_messages(model: &str, messages: &[OpenAIMessage]) -> String {
    let template = detect_template_family(model);
    
    let system = messages.iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.as_str());
    
    // Build history from alternating user/assistant pairs
    let non_system: Vec<_> = messages.iter()
        .filter(|m| m.role != "system")
        .collect();
    
    let mut history = Vec::new();
    let mut i = 0;
    while i + 1 < non_system.len() {
        if non_system[i].role == "user" && non_system[i + 1].role == "assistant" {
            history.push((
                non_system[i].content.clone(),
                non_system[i + 1].content.clone(),
            ));
            i += 2;
        } else {
            i += 1;
        }
    }
    
    // Last user message (if unpaired)
    let user_input = if let Some(last) = non_system.last() {
        if last.role == "user" && (non_system.len() % 2 == 1) {
            Some(last.content.as_str())
        } else {
            None
        }
    } else {
        None
    };
    
    template.render(system, &history, user_input)
}

/// Simple token estimation (~4 chars per token)
fn estimate_tokens(text: &str) -> usize {
    (text.len() + 3) / 4
}
