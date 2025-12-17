use std::{
	collections::{HashMap, HashSet},
	path::PathBuf,
};

use crate::auto_discovery::{DiscoveredModel, ModelAutoDiscovery};

#[derive(Debug, Clone)]
pub struct ModelEntry {
	pub name: String,
	pub base_path: PathBuf,
	pub lora_path: Option<PathBuf>,
	pub template: Option<String>,
	pub ctx_len: Option<usize>,
	pub n_threads: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ModelSpec {
	pub name: String,
	pub base_path: PathBuf,
	pub lora_path: Option<PathBuf>,
	pub template: Option<String>,
	pub ctx_len: Option<usize>,
	pub n_threads: Option<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct Registry {
	pub inner: HashMap<String, ModelEntry>,
	pub discovered_models: HashMap<String, DiscoveredModel>,
}

impl Registry {
	pub fn new() -> Self {
		Self {
			inner: HashMap::new(),
			discovered_models: HashMap::new(),
		}
	}

	pub fn with_discovery() -> Self {
		let discovery = ModelAutoDiscovery::new();
		let discovered_models = discovery.discover_models();
		Self {
			inner: HashMap::new(),
			discovered_models,
		}
	}

	pub fn register(&mut self, entry: ModelEntry) {
		self.inner.insert(entry.name.clone(), entry);
	}

	pub fn auto_register_discovered(&mut self) {
		let discovered = self.discovered_models.clone();
		for (name, model) in discovered {
			if self.inner.contains_key(&name) {
				continue;
			}
			let template = self.infer_template(&name);
			self.inner.insert(
				name.clone(),
				ModelEntry {
					name,
					base_path: model.path,
					lora_path: model.lora_path,
					template,
					ctx_len: None,
					n_threads: None,
				},
			);
		}
	}

	pub fn get(&self, name: &str) -> Option<&ModelEntry> {
		self.inner.get(name)
	}

	pub fn to_spec(&self, name: &str) -> Option<ModelSpec> {
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

		let discovered = self.discovered_models.get(name)?;
		Some(ModelSpec {
			name: discovered.name.clone(),
			base_path: discovered.path.clone(),
			lora_path: discovered.lora_path.clone(),
			template: self.infer_template(name),
			ctx_len: None,
			n_threads: None,
		})
	}

	pub fn list(&self) -> Vec<&ModelEntry> {
		self.inner.values().collect()
	}

	pub fn list_all_available(&self) -> Vec<String> {
		let mut set: HashSet<String> = HashSet::new();
		for k in self.inner.keys() {
			set.insert(k.clone());
		}
		for k in self.discovered_models.keys() {
			set.insert(k.clone());
		}
		let mut v: Vec<String> = set.into_iter().collect();
		v.sort();
		v
	}

	pub fn infer_template(&self, name: &str) -> Option<String> {
		let n = name.to_ascii_lowercase();
		if n.contains("qwen") || n.contains("chatglm") {
			return Some("chatml".to_string());
		}
		if n.contains("llama") {
			if n.contains("llama-3") || n.contains("llama3") {
				return Some("llama3".to_string());
			}
			return Some("llama3".to_string());
		}
		if n.contains("phi") {
			return Some("chatml".to_string());
		}
		None
	}

	pub fn refresh_discovered_models(&mut self) {
		let discovery = ModelAutoDiscovery::new();
		self.discovered_models = discovery.discover_models();
	}
}

impl From<&ModelEntry> for ModelSpec {
	fn from(entry: &ModelEntry) -> Self {
		Self {
			name: entry.name.clone(),
			base_path: entry.base_path.clone(),
			lora_path: entry.lora_path.clone(),
			template: entry.template.clone(),
			ctx_len: entry.ctx_len,
			n_threads: entry.n_threads,
		}
	}
}
