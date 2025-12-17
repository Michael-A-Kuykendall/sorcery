use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Instant};

use clap::Parser;

use shimmy::{
	auto_discovery::{filter_llm_only, ModelAutoDiscovery},
	cli::{Cli, Command},
	engine::{adapter::InferenceEngineAdapter, llama::{LlamaEngine, MoeConfig}, GenOptions, InferenceEngine},
	model_registry::{ModelEntry, Registry},
	server::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	let mut registry = Registry::with_discovery();
	if let Some(model_dirs) = &cli.model_dirs {
		let mut discovery = ModelAutoDiscovery::new();
		for p in model_dirs.split(';').map(str::trim).filter(|s| !s.is_empty()) {
			discovery.add_search_path(PathBuf::from(p));
		}
		registry.discovered_models = discovery.discover_models();
	}

	let engine: Box<dyn InferenceEngine> = if cfg!(feature = "llama") {
		let moe = MoeConfig::from_cli(cli.cpu_moe, cli.n_cpu_moe);
		Box::new(LlamaEngine::new_with_moe(cli.gpu_backend.as_deref(), moe))
	} else {
		Box::new(InferenceEngineAdapter::new())
	};

	match cli.cmd {
		Command::Serve { bind, model_path } => {
			if let Some(p) = model_path {
				let path = PathBuf::from(p);
				let name = path
					.file_stem()
					.and_then(|s| s.to_str())
					.unwrap_or("model")
					.to_string();
				registry.register(ModelEntry {
					name,
					base_path: path,
					lora_path: None,
					template: None,
					ctx_len: None,
					n_threads: None,
				});
			}

			registry.auto_register_discovered();

			let addr = parse_bind(&bind);
			let state = Arc::new(AppState::new(engine, registry));
			shimmy::server::run(addr, state).await
		}

		Command::List { short } => {
			registry.auto_register_discovered();
			if short {
				for name in registry.list_all_available() {
					println!("{}", name);
				}
				return Ok(());
			}

			for name in registry.list_all_available() {
				let spec = registry.to_spec(&name);
				if let Some(spec) = spec {
					println!("{} -> {}", spec.name, spec.base_path.display());
				} else {
					println!("{}", name);
				}
			}
			Ok(())
		}

		Command::Discover { llm_only } => {
			let discovery = ModelAutoDiscovery::new();
			let mut models = discovery.discover_models();
			if llm_only {
				models = filter_llm_only(models);
			}
			let mut names: Vec<_> = models.keys().cloned().collect();
			names.sort();
			for name in names {
				println!("{}", name);
			}
			Ok(())
		}

		Command::Probe { name } => {
			let Some(spec) = registry.to_spec(&name) else {
				anyhow::bail!("Model not found: {}", name);
			};
			engine.load(&spec).await.map_err(|e| anyhow::anyhow!(e))?;
			println!("OK: loaded {}", name);
			Ok(())
		}

		Command::Bench { name, max_tokens } => {
			let Some(spec) = registry.to_spec(&name) else {
				anyhow::bail!("Model not found: {}", name);
			};
			let model = engine.load(&spec).await.map_err(|e| anyhow::anyhow!(e))?;

			let mut opts = GenOptions::default();
			opts.max_tokens = max_tokens;

			let started = Instant::now();
			let _ = model
				.generate("bench", opts, None)
				.await
				.map_err(|e| anyhow::anyhow!(e))?;
			let elapsed = started.elapsed().as_secs_f64();
			let tps = (max_tokens as f64) / elapsed.max(1e-9);
			println!("Bench: {} tokens in {:.3}s ({:.1} tok/s)", max_tokens, elapsed, tps);
			Ok(())
		}

		Command::Generate { name, prompt, max_tokens } => {
			let Some(spec) = registry.to_spec(&name) else {
				anyhow::bail!("Model not found: {}", name);
			};
			let model = engine.load(&spec).await.map_err(|e| anyhow::anyhow!(e))?;
			let mut opts = GenOptions::default();
			opts.max_tokens = max_tokens;
			let text = model
				.generate(&prompt, opts, None)
				.await
				.map_err(|e| anyhow::anyhow!(e))?;
			println!("{}", text);
			Ok(())
		}

		Command::GpuInfo => {
			if cfg!(feature = "llama") {
				let moe = MoeConfig::from_cli(cli.cpu_moe, cli.n_cpu_moe);
				let llama = LlamaEngine::new_with_moe(cli.gpu_backend.as_deref(), moe);
				println!("Backend: {}", llama.get_backend_info());
			} else {
				println!("Backend: unavailable (llama feature disabled)");
			}
			Ok(())
		}

		Command::Init { template, output, name } => {
			println!("Init: template={} output={} name={:?}", template, output, name);
			println!("Template generation is a placeholder in this rehydration.");
			Ok(())
		}
	}
}

fn parse_bind(bind: &str) -> SocketAddr {
	if bind == "auto" {
		return "127.0.0.1:0".parse().unwrap();
	}
	bind.parse().unwrap_or_else(|_| "127.0.0.1:11435".parse().unwrap())
}
