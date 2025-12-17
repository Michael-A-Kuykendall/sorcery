//! Native API - Shimmy's Native JSON API
//! 
//! Rehydrated from: api.spell
//! 
//! Non-OpenAI-compatible API with WebSocket streaming.

use axum::{
    extract::{State, ws::{WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::engine::{GenOptions, InferenceEngine, ModelSpec};
use crate::server::AppState;
use crate::templates::detect_template_family;

// ═══════════════════════════════════════════════════════════════════
// Request/Response Types
// From api.spell: @GenerateRequest, @ChatMessage
// ═══════════════════════════════════════════════════════════════════

/// Text generation request
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_temp")]
    pub temperature: f32,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub top_k: Option<u32>,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub stream: bool,
}

fn default_max_tokens() -> usize { 256 }
fn default_temp() -> f32 { 0.7 }

/// Chat message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion request
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_temp")]
    pub temperature: f32,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub top_k: Option<u32>,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
    #[serde(default)]
    pub stream: bool,
}

/// Generation response
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub text: String,
    pub model: String,
    pub tokens_generated: usize,
    pub finish_reason: String,
}

/// Streaming chunk
#[derive(Debug, Serialize)]
pub struct StreamChunk {
    pub text: String,
    pub done: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Request Handlers
// From api.spell: handlers with model resolution
// ═══════════════════════════════════════════════════════════════════

/// POST /api/generate
/// Text completion endpoint
pub async fn generate_handler<E: InferenceEngine + Send + Sync>(
    State(state): State<Arc<AppState<E>>>,
    Json(req): Json<GenerateRequest>,
) -> impl IntoResponse {
    // Resolve model from registry
    let spec = match state.registry.read().await.to_spec(&req.model) {
        Some(spec) => spec,
        None => {
            return Json(serde_json::json!({
                "error": format!("Model not found: {}", req.model)
            }));
        }
    };
    
    // Build generation options
    let options = GenOptions {
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        top_p: req.top_p.unwrap_or(0.95),
        top_k: req.top_k.unwrap_or(40),
        stop_sequences: req.stop.unwrap_or_default(),
        ..Default::default()
    };
    
    // Load or get cached model
    let model = match state.engine.load(&spec).await {
        Ok(m) => m,
        Err(e) => {
            return Json(serde_json::json!({
                "error": format!("Failed to load model: {}", e)
            }));
        }
    };
    
    // Generate
    match model.generate(&req.prompt, options).await {
        Ok(text) => {
            Json(serde_json::json!(GenerateResponse {
                text: text.clone(),
                model: req.model,
                tokens_generated: text.split_whitespace().count(), // approximate
                finish_reason: "stop".to_string(),
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "error": format!("Generation failed: {}", e)
            }))
        }
    }
}

/// POST /api/chat
/// Chat completion endpoint (native, not OpenAI format)
pub async fn chat_handler<E: InferenceEngine + Send + Sync>(
    State(state): State<Arc<AppState<E>>>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    // Resolve model from registry
    let spec = match state.registry.read().await.to_spec(&req.model) {
        Some(spec) => spec,
        None => {
            return Json(serde_json::json!({
                "error": format!("Model not found: {}", req.model)
            }));
        }
    };
    
    // Format messages into prompt using template
    let template = detect_template_family(&req.model);
    
    let system = req.messages.iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.as_str());
    
    let history: Vec<(String, String)> = req.messages.iter()
        .filter(|m| m.role == "user" || m.role == "assistant")
        .collect::<Vec<_>>()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 && chunk[0].role == "user" && chunk[1].role == "assistant" {
                Some((chunk[0].content.clone(), chunk[1].content.clone()))
            } else {
                None
            }
        })
        .collect();
    
    let user_input = req.messages.iter()
        .rev()
        .find(|m| m.role == "user")
        .map(|m| m.content.as_str());
    
    let prompt = template.render(system, &history, user_input);
    
    // Build generation options
    let options = GenOptions {
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        top_p: req.top_p.unwrap_or(0.95),
        top_k: req.top_k.unwrap_or(40),
        stop_sequences: template.stop_tokens(),
        ..Default::default()
    };
    
    // Load or get cached model
    let model = match state.engine.load(&spec).await {
        Ok(m) => m,
        Err(e) => {
            return Json(serde_json::json!({
                "error": format!("Failed to load model: {}", e)
            }));
        }
    };
    
    // Generate
    match model.generate(&prompt, options).await {
        Ok(text) => {
            Json(serde_json::json!({
                "response": ChatMessage {
                    role: "assistant".to_string(),
                    content: text,
                },
                "model": req.model,
            }))
        }
        Err(e) => {
            Json(serde_json::json!({
                "error": format!("Generation failed: {}", e)
            }))
        }
    }
}

/// GET /api/models
/// List available models
pub async fn list_models_handler<E: InferenceEngine + Send + Sync>(
    State(state): State<Arc<AppState<E>>>,
) -> impl IntoResponse {
    let registry = state.registry.read().await;
    let models = registry.list_all_available();
    
    Json(serde_json::json!({
        "models": models
    }))
}

// ═══════════════════════════════════════════════════════════════════
// WebSocket Streaming
// From api.spell: ws_handler with !callback streaming
// ═══════════════════════════════════════════════════════════════════

/// WebSocket upgrade handler for streaming
pub async fn ws_handler<E: InferenceEngine + Send + Sync + 'static>(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState<E>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws_stream(socket, state))
}

async fn handle_ws_stream<E: InferenceEngine + Send + Sync>(
    mut socket: WebSocket,
    state: Arc<AppState<E>>,
) {
    // Receive request from client
    let msg = match socket.recv().await {
        Some(Ok(msg)) => msg,
        _ => return,
    };
    
    let text = match msg.into_text() {
        Ok(t) => t,
        Err(_) => return,
    };
    
    let req: GenerateRequest = match serde_json::from_str(&text) {
        Ok(r) => r,
        Err(e) => {
            let _ = socket.send(axum::extract::ws::Message::Text(
                format!(r#"{{"error": "{}"}}"#, e)
            )).await;
            return;
        }
    };
    
    // Resolve and load model
    let spec = match state.registry.read().await.to_spec(&req.model) {
        Some(s) => s,
        None => {
            let _ = socket.send(axum::extract::ws::Message::Text(
                r#"{"error": "Model not found"}"#.to_string()
            )).await;
            return;
        }
    };
    
    let model = match state.engine.load(&spec).await {
        Ok(m) => m,
        Err(e) => {
            let _ = socket.send(axum::extract::ws::Message::Text(
                format!(r#"{{"error": "{}"}}"#, e)
            )).await;
            return;
        }
    };
    
    let options = GenOptions {
        max_tokens: req.max_tokens,
        temperature: req.temperature,
        ..Default::default()
    };
    
    // Stream tokens via callback
    let socket = Arc::new(Mutex::new(socket));
    let socket_clone = socket.clone();
    
    let callback = Box::new(move |token: &str| {
        let socket = socket_clone.clone();
        let token = token.to_string();
        tokio::spawn(async move {
            let chunk = StreamChunk {
                text: token,
                done: false,
            };
            let msg = serde_json::to_string(&chunk).unwrap();
            let _ = socket.lock().await.send(axum::extract::ws::Message::Text(msg)).await;
        });
    });
    
    // Generate with streaming
    let _ = model.generate_stream(&req.prompt, options, callback).await;
    
    // Send done signal
    let done_chunk = StreamChunk {
        text: String::new(),
        done: true,
    };
    let done_msg = serde_json::to_string(&done_chunk).unwrap();
    let _ = socket.lock().await.send(axum::extract::ws::Message::Text(done_msg)).await;
}
