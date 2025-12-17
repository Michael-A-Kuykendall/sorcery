use std::path::Path;

use async_trait::async_trait;

use crate::{
	engine::{llama::LlamaEngine, EngineError, InferenceEngine, LoadedModel, Result},
	model_registry::ModelSpec,
};

#[derive(Debug, Clone, Copy)]
pub enum BackendChoice {
	Llama,
	HuggingFace,
	MLX,
	SafeTensors,
	Candle,
}

pub struct InferenceEngineAdapter {
	llama_engine: Option<LlamaEngine>,
	huggingface_engine: Option<StubEngine>,
	mlx_engine: Option<StubEngine>,
	safetensors_engine: Option<StubEngine>,
	candle_engine: Option<StubEngine>,
}

impl InferenceEngineAdapter {
	pub fn new() -> Self {
		Self {
			llama_engine: if cfg!(feature = "llama") { Some(LlamaEngine::new()) } else { None },
			huggingface_engine: if cfg!(feature = "huggingface") { Some(StubEngine::new("huggingface")) } else { None },
			mlx_engine: if cfg!(feature = "mlx") { Some(StubEngine::new("mlx")) } else { None },
			safetensors_engine: Some(StubEngine::new("safetensors")),
			candle_engine: if cfg!(feature = "candle") { Some(StubEngine::new("candle")) } else { None },
		}
	}

	pub fn select_backend(&self, spec: &ModelSpec) -> BackendChoice {
		let ext = Path::new(&spec.base_path)
			.extension()
			.and_then(|s| s.to_str())
			.unwrap_or("")
			.to_ascii_lowercase();

		match ext.as_str() {
			"gguf" => BackendChoice::Llama,
			"safetensors" => BackendChoice::SafeTensors,
			"bin" => BackendChoice::HuggingFace,
			"npz" => BackendChoice::MLX,
			_ => {
				if self.llama_engine.is_some() {
					BackendChoice::Llama
				} else if self.huggingface_engine.is_some() {
					BackendChoice::HuggingFace
				} else {
					BackendChoice::SafeTensors
				}
			}
		}
	}
}

#[async_trait]
impl InferenceEngine for InferenceEngineAdapter {
	async fn load(&self, spec: &ModelSpec) -> Result<Box<dyn LoadedModel>> {
		match self.select_backend(spec) {
			BackendChoice::Llama => {
				let engine = self.llama_engine.as_ref().ok_or_else(|| {
					EngineError::LoadFailed("GGUF requires llama feature".into())
				})?;
				engine.load(spec).await
			}
			BackendChoice::HuggingFace => {
				let engine = self.huggingface_engine.as_ref().ok_or_else(|| {
					EngineError::LoadFailed("PyTorch format requires huggingface feature".into())
				})?;
				engine.load(spec).await
			}
			BackendChoice::MLX => {
				let engine = self.mlx_engine.as_ref().ok_or_else(|| {
					EngineError::LoadFailed("MLX format requires macOS ARM64".into())
				})?;
				engine.load(spec).await
			}
			BackendChoice::SafeTensors => {
				let engine = self.safetensors_engine.as_ref().ok_or_else(|| {
					EngineError::LoadFailed("SafeTensors backend unavailable".into())
				})?;
				engine.load(spec).await
			}
			BackendChoice::Candle => {
				let engine = self.candle_engine.as_ref().ok_or_else(|| {
					EngineError::LoadFailed("Candle backend unavailable".into())
				})?;
				engine.load(spec).await
			}
		}
	}
}

struct StubEngine {
	name: &'static str,
}

impl StubEngine {
	fn new(name: &'static str) -> Self {
		Self { name }
	}
}

#[async_trait]
impl InferenceEngine for StubEngine {
	async fn load(&self, spec: &ModelSpec) -> Result<Box<dyn LoadedModel>> {
		if !Path::new(&spec.base_path).exists() {
			return Err(EngineError::ModelNotFound(spec.name.clone()));
		}
		Ok(Box::new(StubLoaded {
			engine: self.name,
			model: spec.name.clone(),
		}))
	}
}

struct StubLoaded {
	engine: &'static str,
	model: String,
}

#[async_trait]
impl LoadedModel for StubLoaded {
	async fn generate(
		&self,
		prompt: &str,
		opts: crate::engine::GenOptions,
		on_token: Option<Box<dyn Fn(String) + Send>>,
	) -> Result<String> {
		let result = format!("[{}:{}] response: {}", self.model, self.engine, prompt);
		if let Some(cb) = on_token {
			for word in result.split_whitespace().take(opts.max_tokens) {
				cb(format!("{} ", word));
			}
		}
		Ok(result)
	}
}
