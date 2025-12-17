//! HTTP Server - Axum-Based API Server
//! 
//! Rehydrated from: server.spell
//! 
//! Unified HTTP server with health checks, CORS, and graceful shutdown.

use axum::{
    extract::State,
    http::{header, Method},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use crate::api::{chat_handler, generate_handler, list_models_handler, ws_handler};
use crate::engine::InferenceEngine;
use crate::model_registry::Registry;
use crate::openai_compat::{chat_completions, list_models};

// ═══════════════════════════════════════════════════════════════════
// AppState - Shared Server State
// From server.spell: @AppState with ^engine and ^registry
// ═══════════════════════════════════════════════════════════════════

/// Shared application state (thread-safe)
pub struct AppState<E: InferenceEngine> {
    pub engine: Arc<E>,
    pub registry: RwLock<Registry>,
    pub server_name: String,
    pub version: String,
}

impl<E: InferenceEngine> AppState<E> {
    pub fn new(engine: E, registry: Registry) -> Self {
        Self {
            engine: Arc::new(engine),
            registry: RwLock::new(registry),
            server_name: "shimmy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Server Configuration
// From server.spell: @ServerConfig
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub enable_openai_compat: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_origins: vec!["*".to_string()],
            enable_openai_compat: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Routes
// From server.spell: route definitions
// ═══════════════════════════════════════════════════════════════════

/// Build the complete router
/// 
/// Routes:
/// - GET  /health          - Health check
/// - GET  /api/models      - List models (native)
/// - POST /api/generate    - Text generation (native)
/// - POST /api/chat        - Chat completion (native)
/// - GET  /api/ws          - WebSocket streaming
/// - GET  /v1/models       - OpenAI-compatible model list
/// - POST /v1/chat/completions - OpenAI-compatible chat
pub fn build_router<E: InferenceEngine + Send + Sync + Clone + 'static>(
    state: Arc<AppState<E>>,
) -> Router {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        // Health check
        .route("/health", get(health_handler))
        
        // Native API
        .route("/api/models", get(list_models_handler::<E>))
        .route("/api/generate", post(generate_handler::<E>))
        .route("/api/chat", post(chat_handler::<E>))
        .route("/api/ws", get(ws_handler::<E>))
        
        // OpenAI-compatible API
        .route("/v1/models", get(list_models::<E>))
        .route("/v1/chat/completions", post(chat_completions::<E>))
        
        // Shared state and middleware
        .with_state(state)
        .layer(cors)
}

// ═══════════════════════════════════════════════════════════════════
// Health Check
// From server.spell: GET /health -> { status, version }
// ═══════════════════════════════════════════════════════════════════

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "server": "shimmy"
    }))
}

// ═══════════════════════════════════════════════════════════════════
// Server Entrypoint
// From server.spell: run() with graceful shutdown
// ═══════════════════════════════════════════════════════════════════

/// Start the HTTP server
/// 
/// From spell:
/// 1. Binds to config.host:config.port
/// 2. Enables graceful shutdown on SIGTERM
/// 3. Logs startup banner
pub async fn run<E: InferenceEngine + Send + Sync + Clone + 'static>(
    state: Arc<AppState<E>>,
    config: ServerConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = build_router(state.clone());
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   🔮 Shimmy Server (Rehydrated)                               ║");
    println!("║   Version: {}                                             ║", env!("CARGO_PKG_VERSION"));
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║   Native API:   http://{}/api/                   ║", addr);
    println!("║   OpenAI Compat: http://{}/v1/                   ║", addr);
    println!("║   Health Check:  http://{}/health                ║", addr);
    println!("╚═══════════════════════════════════════════════════════════════╝");
    
    // Graceful shutdown handler
    let shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C handler");
        println!("\n🔮 Shimmy shutting down gracefully...");
    };
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════
// Builder Pattern Helper
// ═══════════════════════════════════════════════════════════════════

pub struct ServerBuilder<E: InferenceEngine> {
    engine: Option<E>,
    registry: Option<Registry>,
    config: ServerConfig,
}

impl<E: InferenceEngine> ServerBuilder<E> {
    pub fn new() -> Self {
        Self {
            engine: None,
            registry: None,
            config: ServerConfig::default(),
        }
    }
    
    pub fn engine(mut self, engine: E) -> Self {
        self.engine = Some(engine);
        self
    }
    
    pub fn registry(mut self, registry: Registry) -> Self {
        self.registry = Some(registry);
        self
    }
    
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }
    
    pub fn build(self) -> Result<(Arc<AppState<E>>, ServerConfig), &'static str> {
        let engine = self.engine.ok_or("Engine is required")?;
        let registry = self.registry.unwrap_or_else(Registry::new);
        
        let state = Arc::new(AppState::new(engine, registry));
        Ok((state, self.config))
    }
}

impl<E: InferenceEngine> Default for ServerBuilder<E> {
    fn default() -> Self {
        Self::new()
    }
}
