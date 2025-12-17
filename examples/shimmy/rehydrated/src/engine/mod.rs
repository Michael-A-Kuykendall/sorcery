//! Shimmy Engine Module - Core Traits
//! 
//! Rehydrated from: engine.spell
//! 
//! This module defines the core abstraction layer for inference engines.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod adapter;
pub mod llama;

// ═══════════════════════════════════════════════════════════════════
// GenOptions - Generation Parameters
// From engine.spell: @GenOptions
// ═══════════════════════════════════════════════════════════════════

/// Generation options shared across all backends.
#[derive(Debug, Clone)]
pub struct GenOptions {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,
    pub seed: Option<u64>,
    pub stream: bool,
    pub stop_tokens: Vec<String>,
}

impl Default for GenOptions {
    fn default() -> Self {
        Self {
            max_tokens: 256,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            seed: None,
            stream: false,
            stop_tokens: vec![],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ModelSpec - Model Specification
// From engine.spell (via model_registry.spell): @ModelSpec
// ═══════════════════════════════════════════════════════════════════

/// Specification passed to InferenceEngine::load()
#[derive(Debug, Clone)]
pub struct ModelSpec {
    pub name: String,
    pub base_path: PathBuf,
    pub lora_path: Option<PathBuf>,
    pub template: Option<String>,
    pub ctx_len: Option<usize>,
    pub n_threads: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════
// ModelBackend - Backend Type Enum
// From engine.spell: @ModelBackend
// ═══════════════════════════════════════════════════════════════════

/// Represents the underlying inference backend type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelBackend {
    LlamaGGUF,
    HuggingFace,
    Candle,
    MLX,
    SafeTensors,
}

// ═══════════════════════════════════════════════════════════════════
// InferenceEngine Trait
// From engine.spell: @InferenceEngine
// ═══════════════════════════════════════════════════════════════════

/// The core abstraction for loading models. All backends implement this.
/// 
/// Invariant from spell: implementors must be Send + Sync
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    /// Loads a model from the given specification.
    /// Returns a LoadedModel trait object for generation.
    async fn load(&self, spec: &ModelSpec) -> anyhow::Result<Box<dyn LoadedModel>>;
}

// ═══════════════════════════════════════════════════════════════════
// LoadedModel Trait
// From engine.spell: @LoadedModel
// ═══════════════════════════════════════════════════════════════════

/// A model that has been loaded into memory and is ready to generate.
/// 
/// Invariant from spell: must be thread-safe (Send + Sync)
/// Invariant from spell: on_token callback enables streaming
#[async_trait]
pub trait LoadedModel: Send + Sync {
    /// Generates text from a prompt.
    /// 
    /// If `on_token` is Some, streams each token via the callback.
    /// Returns the full generated text.
    async fn generate(
        &self,
        prompt: &str,
        opts: GenOptions,
        on_token: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> anyhow::Result<String>;
}
