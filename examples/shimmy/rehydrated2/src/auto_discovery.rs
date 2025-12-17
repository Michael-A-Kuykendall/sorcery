use std::{
	collections::HashMap,
	env,
	fs,
	path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredModel {
	pub name: String,
	pub path: PathBuf,
	pub lora_path: Option<PathBuf>,
	pub size_bytes: u64,
	pub model_type: String,
	pub parameter_count: Option<String>,
	pub quantization: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelAutoDiscovery {
	pub search_paths: Vec<PathBuf>,
}

impl ModelAutoDiscovery {
	pub fn new() -> Self {
		let mut search_paths = vec![];

		// 1. ./models/
		if let Ok(cwd) = env::current_dir() {
			search_paths.push(cwd.join("models"));
		}

		// 2. parent of SHIMMY_BASE_GGUF
		if let Ok(p) = env::var("SHIMMY_BASE_GGUF") {
			let pb = PathBuf::from(p);
			if let Some(parent) = pb.parent() {
				search_paths.push(parent.to_path_buf());
			}
		}

		// 3. SHIMMY_MODEL_PATHS (semicolon-separated)
		if let Ok(paths) = env::var("SHIMMY_MODEL_PATHS") {
			for p in paths.split(';').map(str::trim).filter(|s| !s.is_empty()) {
				search_paths.push(PathBuf::from(p));
			}
		}

		// 4. OLLAMA_MODELS env override
		if let Ok(p) = env::var("OLLAMA_MODELS") {
			search_paths.push(PathBuf::from(p));
		}

		// 5-7: common user cache locations
		if let Some(home) = dirs::home_dir() {
			search_paths.push(home.join(".cache").join("huggingface").join("hub"));
			search_paths.push(home.join(".ollama").join("models"));
			search_paths.push(home.join(".lmstudio").join("models"));
		}

		Self { search_paths }
	}

	pub fn add_search_path(&mut self, path: PathBuf) {
		self.search_paths.push(path);
	}

	pub fn discover_models(&self) -> HashMap<String, DiscoveredModel> {
		let mut out = HashMap::new();
		for path in &self.search_paths {
			for model in scan_directory_with_depth(path.clone(), 4) {
				out.entry(model.name.clone()).or_insert(model);
			}
		}
		out
	}
}

pub fn scan_directory_with_depth(path: PathBuf, max_depth: usize) -> Vec<DiscoveredModel> {
	if !path.exists() {
		return vec![];
	}

	let mut models = vec![];

	for entry in WalkDir::new(&path)
		.max_depth(max_depth)
		.follow_links(false)
		.into_iter()
		.filter_map(Result::ok)
	{
		if !entry.file_type().is_file() {
			continue;
		}

		let p = entry.path();
		let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
		let model_type = match ext.as_str() {
			"gguf" => "gguf",
			"safetensors" => "safetensors",
			"bin" => "bin",
			_ => continue,
		};

		let file_name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
		let (name, parameter_count, quantization) = parse_filename(file_name);
		let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
		let lora_path = detect_lora_adapter(p);

		models.push(DiscoveredModel {
			name,
			path: p.to_path_buf(),
			lora_path,
			size_bytes,
			model_type: model_type.to_string(),
			parameter_count,
			quantization,
		});
	}

	group_sharded_models(models)
}

pub fn parse_filename(filename: &str) -> (String, Option<String>, Option<String>) {
	let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(filename);

	// Strip shard suffix: -00001-of-00003
	let shard_stripped = if let Some((base, _)) = stem.rsplit_once("-000") {
		// Only strip if it matches the shard pattern more fully.
		if stem.contains("-of-") {
			// best-effort: remove last 17 chars like -00001-of-00003
			let re = regex::Regex::new(r"-\d{5}-of-\d{5}$").ok();
			if let Some(re) = re {
				re.replace(stem, "").to_string()
			} else {
				base.to_string()
			}
		} else {
			stem.to_string()
		}
	} else {
		stem.to_string()
	};

	let tokens: Vec<&str> = shard_stripped.split('-').filter(|t| !t.is_empty()).collect();
	let mut param_count: Option<String> = None;
	let mut quant: Option<String> = None;

	for t in &tokens {
		if param_count.is_none() && is_param_count(t) {
			param_count = Some(t.to_ascii_uppercase());
		}
		if quant.is_none() && is_quantization(t) {
			quant = Some((*t).to_string());
		}
	}

	let name_tokens: Vec<&str> = tokens
		.into_iter()
		.filter(|t| {
			// Keep param-count token in the visible name, but drop quantization token.
			if let Some(q) = quant.as_deref() {
				if *t == q {
					return false;
				}
			}
			true
		})
		.collect();

	let name = name_tokens.join("-");
	(name, param_count, quant)
}

fn is_param_count(token: &str) -> bool {
	let t = token.trim();
	if t.len() < 2 {
		return false;
	}
	let (num, suffix) = t.split_at(t.len() - 1);
	matches!(suffix, "b" | "B") && num.chars().all(|c| c.is_ascii_digit())
}

fn is_quantization(token: &str) -> bool {
	let t = token.trim();
	if !t.starts_with('Q') {
		return false;
	}
	t.chars().any(|c| c.is_ascii_digit())
}

pub fn group_sharded_models(models: Vec<DiscoveredModel>) -> Vec<DiscoveredModel> {
	let mut by_base: HashMap<String, Vec<DiscoveredModel>> = HashMap::new();
	let shard_re = regex::Regex::new(r"^(?P<base>.+)-\d{5}-of-\d{5}$").ok();

	for m in models {
		let key = if let Some(re) = &shard_re {
			let stem = m
				.path
				.file_stem()
				.and_then(|s| s.to_str())
				.unwrap_or(&m.name);
			re.captures(stem)
				.and_then(|cap| cap.name("base").map(|b| b.as_str().to_string()))
				.unwrap_or_else(|| m.name.clone())
		} else {
			m.name.clone()
		};

		by_base.entry(key).or_default().push(m);
	}

	let mut out = vec![];
	for (_base, mut group) in by_base {
		if group.len() == 1 {
			out.push(group.pop().unwrap());
			continue;
		}

		group.sort_by_key(|m| m.path.clone());
		let mut first = group[0].clone();
		first.size_bytes = group.iter().map(|m| m.size_bytes).sum();
		out.push(first);
	}

	out
}

pub fn discover_ollama_models(ollama_path: PathBuf) -> Vec<DiscoveredModel> {
	let manifests = ollama_path.join("manifests");
	let blobs = ollama_path.join("blobs");
	if !manifests.exists() || !blobs.exists() {
		return vec![];
	}

	let mut out = vec![];
	for entry in WalkDir::new(manifests)
		.max_depth(6)
		.follow_links(false)
		.into_iter()
		.filter_map(Result::ok)
	{
		if !entry.file_type().is_file() {
			continue;
		}
		let text = match fs::read_to_string(entry.path()) {
			Ok(t) => t,
			Err(_) => continue,
		};

		let json: serde_json::Value = match serde_json::from_str(&text) {
			Ok(v) => v,
			Err(_) => continue,
		};

		let digests: Vec<String> = json
			.pointer("/layers")
			.and_then(|v| v.as_array())
			.map(|layers| {
				layers
					.iter()
					.filter_map(|l| l.get("digest").and_then(|d| d.as_str()).map(|s| s.to_string()))
					.collect()
			})
			.unwrap_or_default();

		let digest = match digests.first() {
			Some(d) => d,
			None => continue,
		};

		let blob_name = digest.replace(':', "-");
		let blob_path = blobs.join(blob_name);
		if !blob_path.exists() {
			continue;
		}

		let name = entry
			.path()
			.file_stem()
			.and_then(|s| s.to_str())
			.unwrap_or("ollama-model")
			.to_string();

		let size_bytes = fs::metadata(&blob_path).map(|m| m.len()).unwrap_or(0);
		out.push(DiscoveredModel {
			name,
			path: blob_path,
			lora_path: None,
			size_bytes,
			model_type: "bin".to_string(),
			parameter_count: None,
			quantization: None,
		});
	}

	out
}

pub fn filter_llm_only(models: HashMap<String, DiscoveredModel>) -> HashMap<String, DiscoveredModel> {
	let mut out = HashMap::new();
	for (name, model) in models {
		let n = name.to_ascii_lowercase();
		let blocked = n.contains("clip-")
			|| n.contains("clip_")
			|| n.starts_with("sd-")
			|| n.contains("sdxl")
			|| n.contains("stable-")
			|| n.contains("whisper")
			|| n.contains("vae")
			|| n.contains("encoder")
			|| n.contains("embedding")
			|| n.contains("embed");
		if !blocked {
			out.insert(name, model);
		}
	}
	out
}

fn detect_lora_adapter(model_path: &Path) -> Option<PathBuf> {
	let dir = model_path.parent()?;
	let mut best: Option<PathBuf> = None;
	if let Ok(entries) = fs::read_dir(dir) {
		for e in entries.flatten() {
			let p = e.path();
			if !p.is_file() {
				continue;
			}
			let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
			if ext != "safetensors" {
				continue;
			}
			let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
			if name.contains("lora") || name.contains("adapter") {
				best = Some(p);
				break;
			}
		}
	}
	best
}
