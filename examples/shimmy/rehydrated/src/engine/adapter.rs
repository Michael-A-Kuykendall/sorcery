//! Inference Engine Adapter - Multi-Backend Routing
//! 
//! Rehydrated from: engine_adapter.spell
//! 
//! Routes model loading to the appropriate backend based on file type.

use super::{llama::LlamaEngine, GenOptions, InferenceEngine, LoadedModel, ModelSpec};
use async_trait::async_trait;

// ═══════════════════════════════════════════════════════════════════
// BackendChoice - Internal Routing Enum
// From engine_adapter.spell: @BackendChoice
// ═══════════════════════════════════════════════════════════════════

/// Internal enum for selecting the appropriate engine.
#[derive(Debug, Clone, Copy)]
pub enum BackendChoice {
    Llama,
    HuggingFace,
    MLX,
    SafeTensors,
    Candle,
}

// ═══════════════════════════════════════════════════════════════════
// InferenceEngineAdapter
// From engine_adapter.spell: @InferenceEngineAdapter
// ═══════════════════════════════════════════════════════════════════

/// Routes model loading to the appropriate backend based on file type.
pub struct InferenceEngineAdapter {
    llama_engine: Option<LlamaEngine>,
    // In full impl, would also have:
    // huggingface_engine: Option<HuggingFaceEngine>,
    // mlx_engine: Option<MLXEngine>,
    // safetensors_engine: Option<SafeTensorsEngine>,
}

impl InferenceEngineAdapter {
    pub fn new() -> Self {
        // Initialize available engines based on feature flags
        // From spell: cfg!(feature = "llama") -> Some(LlamaEngine::new())
        
        Self {
            llama_engine: Some(LlamaEngine::new()),
        }
    }

    /// Auto-detection based on file extension and features.
    /// From spell:
    /// .gguf -> Llama (if feature enabled)
    /// .safetensors -> SafeTensors or HuggingFace
    /// .bin -> HuggingFace (PyTorch format)
    /// .npz -> MLX (if macOS ARM64)
    pub fn select_backend(&self, spec: &ModelSpec) -> BackendChoice {
        let extension = spec.base_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension {
            "gguf" => BackendChoice::Llama,
            "safetensors" => BackendChoice::SafeTensors,
            "bin" => BackendChoice::HuggingFace,
            "npz" => BackendChoice::MLX,
            _ => BackendChoice::Llama, // fallback
        }
    }
}

impl Default for InferenceEngineAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InferenceEngine for InferenceEngineAdapter {
    async fn load(&self, spec: &ModelSpec) -> anyhow::Result<Box<dyn LoadedModel>> {
        let backend = self.select_backend(spec);
        
        match backend {
            BackendChoice::Llama => {
                if let Some(ref engine) = self.llama_engine {
                    engine.load(spec).await
                } else {
                    anyhow::bail!("GGUF requires llama feature")
                }
            }
            BackendChoice::HuggingFace => {
                anyhow::bail!("HuggingFace backend not implemented in rehydration")
            }
            BackendChoice::MLX => {
                anyhow::bail!("MLX backend requires macOS ARM64")
            }
            BackendChoice::SafeTensors => {
                // Fallback to Llama for now
                if let Some(ref engine) = self.llama_engine {
                    engine.load(spec).await
                } else {
                    anyhow::bail!("No backend available for SafeTensors")
                }
            }
            BackendChoice::Candle => {
                anyhow::bail!("Candle backend not implemented in rehydration")
            }
        }
    }
}
