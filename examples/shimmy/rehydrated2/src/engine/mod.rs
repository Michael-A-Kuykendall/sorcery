use async_trait::async_trait;

use crate::model_registry::ModelSpec;

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

#[derive(Debug, Clone, Copy)]
pub enum ModelBackend {
	LlamaGGUF,
	HuggingFace,
	Candle,
	MLX,
	SafeTensors,
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
	#[error("model not found: {0}")]
	ModelNotFound(String),
	#[error("model load failed: {0}")]
	LoadFailed(String),
	#[error("out of memory")]
	OutOfMemory,
	#[error("generation failed: {0}")]
	GenerationFailed(String),
}

pub type Result<T> = std::result::Result<T, EngineError>;

#[async_trait]
pub trait InferenceEngine: Send + Sync {
	async fn load(&self, spec: &ModelSpec) -> Result<Box<dyn LoadedModel>>;
}

#[async_trait]
pub trait LoadedModel: Send + Sync {
	async fn generate(
		&self,
		prompt: &str,
		opts: GenOptions,
		on_token: Option<Box<dyn Fn(String) + Send>>,
	) -> Result<String>;
}

pub mod adapter;
pub mod llama;
