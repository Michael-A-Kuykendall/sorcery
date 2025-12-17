use std::{net::SocketAddr, sync::Arc, time::Instant};

use axum::{
	extract::State,
	body::Body,
	http::{header, Method, Request, StatusCode},
	middleware::Next,
	response::{IntoResponse, Response},
	routing::{get, post},
	Json, Router,
};
use serde_json::{json, Value};

use crate::{
	engine::InferenceEngine,
	model_registry::Registry,
};

pub struct ObservabilityManager {
	started_at: Instant,
}

impl Default for ObservabilityManager {
	fn default() -> Self {
		Self {
			started_at: Instant::now(),
		}
	}
}

impl ObservabilityManager {
	pub fn uptime_seconds(&self) -> u64 {
		self.started_at.elapsed().as_secs()
	}
}

#[derive(Default)]
pub struct ResponseCache;

pub struct AppState {
	pub engine: Box<dyn InferenceEngine>,
	pub registry: tokio::sync::RwLock<Registry>,
	pub observability: ObservabilityManager,
	pub response_cache: ResponseCache,
}

impl AppState {
	pub fn new(engine: Box<dyn InferenceEngine>, registry: Registry) -> Self {
		Self {
			engine,
			registry: tokio::sync::RwLock::new(registry),
			observability: ObservabilityManager::default(),
			response_cache: ResponseCache::default(),
		}
	}
}

pub async fn run(addr: SocketAddr, state: Arc<AppState>) -> anyhow::Result<()> {
	let app = Router::new()
		.route("/health", get(health_check))
		.route("/metrics", get(metrics_endpoint))
		.route("/diag", get(diag_handler))
		// Native Shimmy API
		.route("/api/generate", post(crate::api::generate))
		.route("/api/models", get(crate::api::list_models))
		.route("/api/models/discover", post(crate::api::discover_models))
		.route("/api/models/:name/load", post(crate::api::load_model))
		.route("/api/models/:name/unload", post(crate::api::unload_model))
		.route("/api/models/:name/status", get(crate::api::model_status))
		.route("/api/tools", get(crate::api::list_tools))
		.route("/api/tools/:name/execute", post(crate::api::execute_tool))
		.route("/api/workflows/execute", post(crate::api::execute_workflow))
		// WebSocket
		.route("/ws/generate", get(crate::api::ws_generate))
		// OpenAI compatible
		.route("/v1/chat/completions", post(crate::openai_compat::chat_completions))
		.route("/v1/models", get(crate::openai_compat::models))
		// Anthropic compatible (stub)
		.route("/v1/messages", post(crate::anthropic_compat::messages))
		.with_state(state)
		.layer(axum::middleware::from_fn(cors_layer));

	let listener = tokio::net::TcpListener::bind(addr).await?;
	let actual = listener.local_addr()?;
	println!("✅ Ready to serve requests");
	println!("   • POST /api/generate (streaming + non-streaming)");
	println!("   • GET  /health (health check + metrics)");
	println!("   • GET  /v1/models (OpenAI-compatible)");
	println!("   • POST /v1/chat/completions (OpenAI-compatible)");
	println!("Listening on http://{}", actual);

	axum::serve(listener, app).await?;
	Ok(())
}

pub async fn cors_layer(req: Request<Body>, next: Next) -> Response {
	if req.method() == Method::OPTIONS {
		let mut res = StatusCode::OK.into_response();
		add_cors_headers(res.headers_mut());
		return res;
	}

	let mut res = next.run(req).await;
	add_cors_headers(res.headers_mut());
	res
}

fn add_cors_headers(headers: &mut axum::http::HeaderMap) {
	headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
	headers.insert(
		header::ACCESS_CONTROL_ALLOW_METHODS,
		"GET, POST, OPTIONS".parse().unwrap(),
	);
	headers.insert(
		header::ACCESS_CONTROL_ALLOW_HEADERS,
		"Content-Type, Authorization".parse().unwrap(),
	);
	headers.insert(header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());
}

pub async fn health_check(State(state): State<Arc<AppState>>) -> Json<Value> {
	let reg = state.registry.read().await;
	let manual = reg.inner.len();
	let discovered = reg.discovered_models.len();
	let total = reg.list_all_available().len();

	Json(json!({
		"status": "ok",
		"service": "shimmy",
		"version": env!("CARGO_PKG_VERSION"),
		"models": {
			"total": total,
			"discovered": discovered,
			"manual": manual
		},
		"endpoints": {
			"health": "/health",
			"models": "/v1/models",
			"chat": "/v1/chat/completions",
			"generate": "/api/generate"
		},
		"compatibility": {
			"openai": true,
			"cors": true
		},
		"timestamp": chrono::Utc::now().to_rfc3339(),
		"uptime_seconds": state.observability.uptime_seconds()
	}))
}

pub async fn metrics_endpoint(State(state): State<Arc<AppState>>) -> Json<Value> {
	let reg = state.registry.read().await;
	let total_count = reg.list_all_available().len();
	let discovered = reg.discovered_models.len();
	let manual = reg.inner.len();

	let mut total_size_mb = 0.0_f64;
	for d in reg.discovered_models.values() {
		total_size_mb += (d.size_bytes as f64) / (1024.0 * 1024.0);
	}

	Json(json!({
		"models": {
			"total_count": total_count,
			"total_size_mb": total_size_mb,
			"by_type": { "discovered": discovered, "manual": manual }
		},
		"system": {
			"memory_total_mb": 0,
			"memory_free_mb": 0,
			"memory_available_mb": 0
		},
		"features": {
			"llama": cfg!(feature = "llama"),
			"huggingface": cfg!(feature = "huggingface")
		},
		"endpoints": ["/health", "/metrics", "/diag", "/api/generate", "/v1/models", "/v1/chat/completions"],
		"timestamp": chrono::Utc::now().to_rfc3339()
	}))
}

pub async fn diag_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
	let reg = state.registry.read().await;
	Json(json!({
		"service": "shimmy",
		"version": env!("CARGO_PKG_VERSION"),
		"models": reg.list_all_available(),
		"features": {
			"llama": cfg!(feature = "llama"),
			"gpu": cfg!(feature = "gpu")
		}
	}))
}
