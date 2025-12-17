//! Auto Discovery - Filesystem Model Discovery
//! 
//! Rehydrated from: auto_discovery.spell
//! 
//! Scans multiple locations for model files.

use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;

// ═══════════════════════════════════════════════════════════════════
// DiscoveredModel - Metadata from Filesystem
// From auto_discovery.spell: @DiscoveredModel
// ═══════════════════════════════════════════════════════════════════

/// Metadata extracted from filesystem scan without loading.
#[derive(Debug, Clone)]
pub struct DiscoveredModel {
    pub name: String,
    pub path: PathBuf,
    pub lora_path: Option<PathBuf>,
    pub size_bytes: u64,
    pub model_type: String,
    pub parameter_count: Option<String>,
    pub quantization: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════
// ModelAutoDiscovery
// From auto_discovery.spell: @ModelAutoDiscovery
// ═══════════════════════════════════════════════════════════════════

/// The discovery engine that scans multiple locations.
pub struct ModelAutoDiscovery {
    search_paths: Vec<PathBuf>,
}

impl ModelAutoDiscovery {
    /// Creates with default search paths from spell:
    /// 1. ./models/
    /// 2. parent of SHIMMY_BASE_GGUF
    /// 3. SHIMMY_MODEL_PATHS (semicolon-separated)
    /// 4. OLLAMA_MODELS
    /// 5. ~/.cache/huggingface/hub/
    /// 6. ~/.ollama/models/
    /// 7. ~/.lmstudio/models/
    pub fn new() -> Self {
        let mut paths = Vec::new();
        
        // 1. Project local
        paths.push(PathBuf::from("./models"));
        
        // 2. Parent of SHIMMY_BASE_GGUF
        if let Ok(base) = std::env::var("SHIMMY_BASE_GGUF") {
            if let Some(parent) = PathBuf::from(&base).parent() {
                paths.push(parent.to_path_buf());
            }
        }
        
        // 3. SHIMMY_MODEL_PATHS
        if let Ok(model_paths) = std::env::var("SHIMMY_MODEL_PATHS") {
            for path in model_paths.split(';') {
                if !path.is_empty() {
                    paths.push(PathBuf::from(path));
                }
            }
        }
        
        // 4. OLLAMA_MODELS
        if let Ok(ollama) = std::env::var("OLLAMA_MODELS") {
            paths.push(PathBuf::from(ollama));
        }
        
        // 5-7. Default locations
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join(".cache/huggingface/hub"));
            paths.push(home.join(".ollama/models"));
            paths.push(home.join(".lmstudio/models"));
        }
        
        Self { search_paths: paths }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    /// Iterates all search_paths, scans each, merges results
    pub fn discover_models(&self) -> HashMap<String, DiscoveredModel> {
        let mut models = HashMap::new();
        
        for path in &self.search_paths {
            if path.exists() {
                let found = self.scan_directory_with_depth(path, 4);
                for model in found {
                    // First found wins on name collision
                    if !models.contains_key(&model.name) {
                        models.insert(model.name.clone(), model);
                    }
                }
            }
        }
        
        models
    }

    /// Scan directory with max depth limit
    /// From spell: max_depth = 4 (prevents infinite recursion)
    fn scan_directory_with_depth(&self, path: &PathBuf, max_depth: usize) -> Vec<DiscoveredModel> {
        let mut models = Vec::new();
        
        let walker = WalkDir::new(path)
            .max_depth(max_depth)
            .follow_links(true);
        
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let file_path = entry.path();
            
            if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
                let model_type = match ext {
                    "gguf" => Some("gguf"),
                    "safetensors" => Some("safetensors"),
                    "bin" => Some("bin"),
                    _ => None,
                };
                
                if let Some(model_type) = model_type {
                    if let Some(file_name) = file_path.file_stem().and_then(|s| s.to_str()) {
                        let (name, param_count, quant) = parse_filename(file_name);
                        
                        let size_bytes = entry.metadata()
                            .map(|m| m.len())
                            .unwrap_or(0);
                        
                        // Check for LoRA in same directory
                        let lora_path = file_path.parent()
                            .and_then(|parent| {
                                let lora_name = format!("{}-lora.gguf", name);
                                let lora = parent.join(&lora_name);
                                if lora.exists() { Some(lora) } else { None }
                            });
                        
                        models.push(DiscoveredModel {
                            name,
                            path: file_path.to_path_buf(),
                            lora_path,
                            size_bytes,
                            model_type: model_type.to_string(),
                            parameter_count: param_count,
                            quantization: quant,
                        });
                    }
                }
            }
        }
        
        // Group sharded models
        group_sharded_models(models)
    }
}

impl Default for ModelAutoDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse filename for metadata
/// From spell: extracts name, parameter count, quantization
fn parse_filename(filename: &str) -> (String, Option<String>, Option<String>) {
    // Examples:
    // "llama-3-8b-instruct-Q4_K_M" -> ("llama-3-8b-instruct", Some("8B"), Some("Q4_K_M"))
    // "phi3-mini-4k-instruct" -> ("phi3-mini-4k-instruct", None, None)
    
    let parts: Vec<&str> = filename.split('-').collect();
    let mut name_parts = Vec::new();
    let mut param_count = None;
    let mut quantization = None;
    
    for part in parts {
        let upper = part.to_uppercase();
        
        // Check for quantization (Q4_K_M, Q8_0, etc.)
        if upper.starts_with('Q') && upper.chars().nth(1).map(|c| c.is_numeric()).unwrap_or(false) {
            quantization = Some(upper);
            continue;
        }
        
        // Check for parameter count (7B, 8B, 13B, etc.)
        if upper.ends_with('B') && upper.len() <= 4 {
            if upper[..upper.len()-1].chars().all(|c| c.is_numeric() || c == '.') {
                param_count = Some(upper);
                continue;
            }
        }
        
        name_parts.push(part);
    }
    
    (name_parts.join("-"), param_count, quantization)
}

/// Combine multi-part model files
/// From spell: model-00001-of-00003.gguf -> single entry
fn group_sharded_models(models: Vec<DiscoveredModel>) -> Vec<DiscoveredModel> {
    // Simple implementation: just return as-is for now
    // Full implementation would detect -00001-of-00003 patterns
    models
}

/// LLM-only filter
/// From spell: removes CLIP, SD, whisper, VAE, embedding models
pub fn filter_llm_only(models: HashMap<String, DiscoveredModel>) -> HashMap<String, DiscoveredModel> {
    models.into_iter()
        .filter(|(name, _)| {
            let name_lower = name.to_lowercase();
            !name_lower.contains("clip") &&
            !name_lower.contains("sd-") &&
            !name_lower.contains("sdxl") &&
            !name_lower.contains("stable-") &&
            !name_lower.contains("whisper") &&
            !name_lower.contains("vae") &&
            !name_lower.contains("encoder") &&
            !name_lower.contains("embedding") &&
            !name_lower.contains("embed")
        })
        .collect()
}

// Helper for home directory (simple cross-platform)
mod dirs {
    use std::path::PathBuf;
    
    pub fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .map(PathBuf::from)
    }
}
