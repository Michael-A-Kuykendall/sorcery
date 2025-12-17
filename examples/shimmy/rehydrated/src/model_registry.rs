//! Model Registry - Dual-Source Model Hub
//! 
//! Rehydrated from: model_registry.spell
//! 
//! Maintains two sources:
//! 1. inner: manually registered models (priority)
//! 2. discovered_models: auto-discovered from filesystem

use crate::auto_discovery::{DiscoveredModel, ModelAutoDiscovery};
use crate::engine::ModelSpec;
use std::collections::HashMap;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════
// ModelEntry - Manual Registration
// From model_registry.spell: @ModelEntry
// ═══════════════════════════════════════════════════════════════════

/// A manually registered model with explicit configuration.
#[derive(Debug, Clone)]
pub struct ModelEntry {
    pub name: String,
    pub base_path: PathBuf,
    pub lora_path: Option<PathBuf>,
    pub template: Option<String>,
    pub ctx_len: Option<usize>,
    pub n_threads: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════
// Registry - Dual-Source Model Hub
// From model_registry.spell: @Registry
// ═══════════════════════════════════════════════════════════════════

/// The registry maintains two sources with priority ordering.
/// 
/// Invariant from spell: manual entries take priority over discovered
#[derive(Debug, Default)]
pub struct Registry {
    /// Manually registered models (HIGH priority)
    inner: HashMap<String, ModelEntry>,
    /// Auto-discovered models (LOW priority)
    pub discovered_models: HashMap<String, DiscoveredModel>,
}

impl Registry {
    /// Creates empty registry
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            discovered_models: HashMap::new(),
        }
    }

    /// Creates registry with auto-discovery enabled
    pub fn with_discovery() -> Self {
        let discovery = ModelAutoDiscovery::new();
        let discovered = discovery.discover_models();
        
        Self {
            inner: HashMap::new(),
            discovered_models: discovered,
        }
    }

    /// Register a model entry (overwrites existing)
    pub fn register(&mut self, entry: ModelEntry) {
        self.inner.insert(entry.name.clone(), entry);
    }

    /// Promotes all discovered_models to inner registry
    /// Skipped if already in inner (manual takes priority)
    pub fn auto_register_discovered(&mut self) {
        for (name, discovered) in &self.discovered_models {
            if !self.inner.contains_key(name) {
                let template = self.infer_template(name);
                self.inner.insert(name.clone(), ModelEntry {
                    name: name.clone(),
                    base_path: discovered.path.clone(),
                    lora_path: discovered.lora_path.clone(),
                    template,
                    ctx_len: None,
                    n_threads: None,
                });
            }
        }
    }

    /// Get a manually registered model by name
    pub fn get(&self, name: &str) -> Option<&ModelEntry> {
        self.inner.get(name)
    }

    /// Convert to ModelSpec for engine loading
    /// 
    /// From spell: first checks inner (manual), then checks discovered_models
    pub fn to_spec(&self, name: &str) -> Option<ModelSpec> {
        // Check manual first (priority)
        if let Some(entry) = self.inner.get(name) {
            return Some(ModelSpec {
                name: entry.name.clone(),
                base_path: entry.base_path.clone(),
                lora_path: entry.lora_path.clone(),
                template: entry.template.clone(),
                ctx_len: entry.ctx_len,
                n_threads: entry.n_threads,
            });
        }
        
        // Then check discovered
        if let Some(discovered) = self.discovered_models.get(name) {
            return Some(ModelSpec {
                name: discovered.name.clone(),
                base_path: discovered.path.clone(),
                lora_path: discovered.lora_path.clone(),
                template: self.infer_template(name),
                ctx_len: None,
                n_threads: None,
            });
        }
        
        None
    }

    /// Returns manually registered models only
    pub fn list(&self) -> Vec<&ModelEntry> {
        self.inner.values().collect()
    }

    /// Returns all available model names (manual + discovered, deduplicated)
    pub fn list_all_available(&self) -> Vec<String> {
        let mut names: Vec<String> = self.inner.keys().cloned().collect();
        for name in self.discovered_models.keys() {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
        names.sort();
        names
    }

    /// Infer template from model name patterns
    /// From spell:
    /// "qwen" | "chatglm" -> "chatml"
    /// "llama" | "llama-3" -> "llama3"
    /// "phi" -> "chatml"
    pub fn infer_template(&self, name: &str) -> Option<String> {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("qwen") || name_lower.contains("chatglm") || name_lower.contains("phi") {
            Some("chatml".to_string())
        } else if name_lower.contains("llama") {
            Some("llama3".to_string())
        } else {
            None
        }
    }

    /// Re-run auto-discovery scan
    pub fn refresh_discovered_models(&mut self) {
        let discovery = ModelAutoDiscovery::new();
        self.discovered_models = discovery.discover_models();
    }
}
