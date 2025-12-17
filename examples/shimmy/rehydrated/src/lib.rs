//! Shimmy - Local LLM Inference Server (Rehydrated)
//! 
//! ╔══════════════════════════════════════════════════════════════════════╗
//! ║  This crate was REHYDRATED from Sorcery Doctrine spell files.        ║
//! ║  Original source code was NOT consulted during reconstruction.       ║
//! ║                                                                      ║
//! ║  Proof that architectural notation is sufficient for reconstruction. ║
//! ╚══════════════════════════════════════════════════════════════════════╝
//!
//! ## Architecture
//!
//! ```
//!                    ┌─────────────────┐
//!                    │       CLI       │
//!                    │    (clap.rs)    │
//!                    └────────┬────────┘
//!                             │
//!              ┌──────────────┼──────────────┐
//!              │              │              │
//!              ▼              ▼              ▼
//!        ┌──────────┐  ┌──────────┐  ┌──────────┐
//!        │  Serve   │  │ Generate │  │  Models  │
//!        └────┬─────┘  └────┬─────┘  └────┬─────┘
//!             │             │             │
//!             ▼             │             │
//!       ┌──────────┐        │             │
//!       │  Server  │        │             │
//!       │  (Axum)  │        │             │
//!       └────┬─────┘        │             │
//!            │              │             │
//!   ┌────────┴────────┐     │             │
//!   ▼                 ▼     │             │
//! /api/*           /v1/*    │             │
//! (Native)       (OpenAI)   │             │
//!   │                 │     │             │
//!   └────────┬────────┘     │             │
//!            ▼              ▼             ▼
//!       ┌────────────────────────────────────┐
//!       │           Model Registry           │
//!       │     (Manual + Auto-Discovered)     │
//!       └─────────────────┬──────────────────┘
//!                         │
//!       ┌─────────────────┼─────────────────┐
//!       │                 │                 │
//!       ▼                 ▼                 ▼
//!  ┌─────────┐     ┌─────────────┐   ┌──────────┐
//!  │  Llama  │     │ SafeTensors │   │  Future  │
//!  │ Backend │     │   Backend   │   │ Backends │
//!  │ (GGUF)  │     │    (?)      │   │          │
//!  └────┬────┘     └─────────────┘   └──────────┘
//!       │
//!       ▼
//!  ┌─────────────────────────────────────────┐
//!  │            llama.cpp FFI                │
//!  ├─────────┬─────────┬─────────┬───────────┤
//!  │  CUDA   │ Vulkan  │ OpenCL  │   MLX     │
//!  │ (NVIDIA)│  (AMD)  │  (Any)  │  (Apple)  │
//!  └─────────┴─────────┴─────────┴───────────┘
//! ```

pub mod auto_discovery;
pub mod cli;
pub mod engine;
pub mod model_registry;
pub mod templates;

// API modules
pub mod api;
pub mod openai_compat;
pub mod server;

// Re-exports
pub use cli::Cli;
pub use engine::{GenOptions, InferenceEngine, LoadedModel, ModelSpec};
pub use model_registry::Registry;
pub use server::{AppState, ServerConfig};
