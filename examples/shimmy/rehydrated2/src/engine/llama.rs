use std::{path::Path, sync::Mutex};

use async_trait::async_trait;

type BoxTokenCb = Option<Box<dyn Fn(String) + Send>>;

use crate::{
	engine::{EngineError, GenOptions, InferenceEngine, LoadedModel, Result},
	model_registry::ModelSpec,
};

#[derive(Debug, Clone, Copy)]
pub enum GpuBackend {
	Cpu,
	Cuda,
	Vulkan,
	OpenCL,
	Metal,
}

#[derive(Debug, Clone)]
pub struct MoeConfig {
	pub enabled: bool,
	pub offload_all: bool,
	pub n_layers_cpu: Option<usize>,
}

impl Default for MoeConfig {
	fn default() -> Self {
		Self {
			enabled: false,
			offload_all: false,
			n_layers_cpu: None,
		}
	}
}

impl MoeConfig {
	pub fn from_cli(cpu_moe: bool, n_cpu_moe: Option<usize>) -> Self {
		let mut cfg = MoeConfig::default();
		cfg.enabled = cpu_moe || n_cpu_moe.is_some();
		cfg.offload_all = cpu_moe;
		cfg.n_layers_cpu = n_cpu_moe;
		cfg
	}
}

pub struct LlamaEngine {
	backend: GpuBackend,
	moe_config: MoeConfig,
}

impl LlamaEngine {
	pub fn new() -> Self {
		Self {
			backend: detect_gpu_backend(),
			moe_config: MoeConfig::default(),
		}
	}

	pub fn new_with_backend(backend: Option<&str>) -> Self {
		let backend = match backend.unwrap_or("auto").to_ascii_lowercase().as_str() {
			"auto" => detect_gpu_backend(),
			"cpu" => GpuBackend::Cpu,
			"cuda" => GpuBackend::Cuda,
			"vulkan" => GpuBackend::Vulkan,
			"opencl" => GpuBackend::OpenCL,
			_ => detect_gpu_backend(),
		};

		Self {
			backend,
			moe_config: MoeConfig::default(),
		}
	}

	pub fn new_with_moe(backend: Option<&str>, moe_config: MoeConfig) -> Self {
		let mut s = Self::new_with_backend(backend);
		s.moe_config = moe_config;
		s
	}

	pub fn get_backend_info(&self) -> String {
		match self.backend {
			GpuBackend::Cpu => "CPU".to_string(),
			GpuBackend::Cuda => "CUDA".to_string(),
			GpuBackend::Vulkan => "Vulkan".to_string(),
			GpuBackend::OpenCL => "OpenCL".to_string(),
			GpuBackend::Metal => "Metal".to_string(),
		}
	}
}

#[async_trait]
impl InferenceEngine for LlamaEngine {
	async fn load(&self, spec: &ModelSpec) -> Result<Box<dyn LoadedModel>> {
		if !Path::new(&spec.base_path).exists() {
			return Err(EngineError::ModelNotFound(spec.name.clone()));
		}

		// Placeholder for real llama.cpp loading. We keep structure to match the spell.
		let loaded = LlamaLoaded {
			_guard: Mutex::new(()),
			n_ctx: spec.ctx_len.unwrap_or(4096),
			n_threads: spec.n_threads.unwrap_or(8),
			model_name: spec.name.clone(),
			backend: self.backend,
			moe_enabled: self.moe_config.enabled,
		};
		Ok(Box::new(loaded))
	}
}

pub struct LlamaLoaded {
	_guard: Mutex<()>,
	n_ctx: usize,
	n_threads: i32,
	model_name: String,
	backend: GpuBackend,
	moe_enabled: bool,
}

#[async_trait]
impl LoadedModel for LlamaLoaded {
	async fn generate(&self, prompt: &str, opts: GenOptions, on_token: BoxTokenCb) -> Result<String> {
		let _lock = self._guard.lock().map_err(|_| EngineError::GenerationFailed("mutex poisoned".into()))?;

		// Minimal deterministic-ish placeholder text.
		let mut completion = format!("[{}:{}] ", self.model_name, backend_tag(self.backend));
		if self.moe_enabled {
			completion.push_str("(moe) ");
		}
		completion.push_str("response: ");

		// Very simple "generation": echo a bounded slice of the prompt.
		let take = prompt.chars().take(200).collect::<String>();
		completion.push_str(&take);

		// Apply stop tokens by truncation if any match.
		for stop in &opts.stop_tokens {
			if stop.is_empty() {
				continue;
			}
			if let Some(idx) = completion.find(stop) {
				completion.truncate(idx);
				break;
			}
		}

		// Simulate token streaming.
		if let Some(cb) = on_token {
			for word in completion.split_whitespace().take(opts.max_tokens) {
				cb(format!("{} ", word));
			}
		}

		Ok(completion)
	}
}

fn backend_tag(b: GpuBackend) -> &'static str {
	match b {
		GpuBackend::Cpu => "cpu",
		GpuBackend::Cuda => "cuda",
		GpuBackend::Vulkan => "vulkan",
		GpuBackend::OpenCL => "opencl",
		GpuBackend::Metal => "metal",
	}
}

fn detect_gpu_backend() -> GpuBackend {
	// Spell describes probing external tools; keep simple + deterministic.
	if cfg!(feature = "llama-cuda") || cfg!(feature = "gpu") {
		return GpuBackend::Cuda;
	}
	if cfg!(feature = "llama-vulkan") || cfg!(feature = "gpu") {
		return GpuBackend::Vulkan;
	}
	if cfg!(feature = "llama-opencl") || cfg!(feature = "gpu") {
		return GpuBackend::OpenCL;
	}
	GpuBackend::Cpu
}
