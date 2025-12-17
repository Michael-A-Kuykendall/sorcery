//! Shimmy - Main Entry Point (Rehydrated)
//! 
//! Rehydrated from: All spell files combined
//! 
//! This binary was reconstructed from Sorcery notation without
//! consulting the original source code.

use clap::Parser;
use std::sync::Arc;

use shimmy_rehydrated::{
    cli::{Cli, Command, GpuBackendArg},
    engine::{adapter::InferenceEngineAdapter, llama::{LlamaEngine, GpuBackend, MoeConfig}},
    model_registry::Registry,
    server::{run, ServerConfig, AppState},
    auto_discovery::ModelAutoDiscovery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    
    // Set up logging based on verbosity
    match cli.verbose {
        0 => {} // default
        1 => println!("Verbose output enabled"),
        _ => println!("Extra verbose output enabled"),
    }
    
    match cli.command {
        Command::Serve(args) => {
            // Build engine with GPU backend
            let gpu_backend = args.gpu
                .map(|g| g.to_engine_backend())
                .unwrap_or(GpuBackend::Auto);
            
            let engine = if let Some(moe_layers) = args.moe_cpu_offload {
                let moe = MoeConfig::from_cli(
                    Some(moe_layers),
                    args.threads,
                );
                LlamaEngine::new_with_moe(gpu_backend, moe)
            } else {
                LlamaEngine::new_with_backend(gpu_backend)
            };
            
            // Build registry
            let mut registry = if args.auto_discover {
                Registry::with_discovery()
            } else {
                Registry::new()
            };
            
            // Promote discovered to manual
            registry.auto_register_discovered();
            
            // Build server config
            let config = ServerConfig {
                host: args.host,
                port: args.port,
                cors_origins: vec!["*".to_string()],
                enable_openai_compat: !args.no_openai_compat,
            };
            
            // Create state and run
            let state = Arc::new(AppState::new(engine, registry));
            run(state, config).await?;
        }
        
        Command::Generate(args) => {
            // One-shot generation
            let gpu_backend = args.gpu
                .map(|g| g.to_engine_backend())
                .unwrap_or(GpuBackend::Auto);
            
            let engine = LlamaEngine::new_with_backend(gpu_backend);
            let mut registry = Registry::with_discovery();
            registry.auto_register_discovered();
            
            let spec = registry.to_spec(&args.model)
                .ok_or_else(|| format!("Model not found: {}", args.model))?;
            
            let model = engine.load(&spec).await?;
            
            let options = shimmy_rehydrated::GenOptions {
                max_tokens: args.max_tokens,
                temperature: args.temperature,
                top_p: args.top_p.unwrap_or(0.95),
                top_k: args.top_k.unwrap_or(40),
                ..Default::default()
            };
            
            let result = model.generate(&args.prompt, options).await?;
            
            match args.format {
                shimmy_rehydrated::cli::OutputFormat::Text => println!("{}", result),
                shimmy_rehydrated::cli::OutputFormat::Json => {
                    println!("{}", serde_json::json!({
                        "model": args.model,
                        "prompt": args.prompt,
                        "response": result,
                    }));
                }
                shimmy_rehydrated::cli::OutputFormat::Markdown => {
                    println!("## Response\n\n{}", result);
                }
            }
        }
        
        Command::Chat(args) => {
            println!("🔮 Interactive chat mode");
            println!("Model: {}", args.model);
            println!("Type 'exit' to quit\n");
            
            let gpu_backend = args.gpu
                .map(|g| g.to_engine_backend())
                .unwrap_or(GpuBackend::Auto);
            
            let engine = LlamaEngine::new_with_backend(gpu_backend);
            let mut registry = Registry::with_discovery();
            registry.auto_register_discovered();
            
            let spec = registry.to_spec(&args.model)
                .ok_or_else(|| format!("Model not found: {}", args.model))?;
            
            let model = engine.load(&spec).await?;
            
            let template = shimmy_rehydrated::templates::detect_template_family(&args.model);
            let system = args.system.as_deref();
            let mut history: Vec<(String, String)> = Vec::new();
            
            loop {
                print!("> ");
                use std::io::Write;
                std::io::stdout().flush()?;
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                let input = input.trim();
                
                if input == "exit" || input == "quit" {
                    break;
                }
                
                let prompt = template.render(system, &history, Some(input));
                
                let options = shimmy_rehydrated::GenOptions {
                    max_tokens: 512,
                    temperature: args.temperature,
                    stop_sequences: template.stop_tokens(),
                    ..Default::default()
                };
                
                let response = model.generate(&prompt, options).await?;
                println!("\n{}\n", response);
                
                history.push((input.to_string(), response));
            }
        }
        
        Command::Models(args) => {
            let mut registry = Registry::with_discovery();
            registry.auto_register_discovered();
            
            let models = registry.list_all_available();
            
            match args.format {
                shimmy_rehydrated::cli::ListFormat::Table => {
                    println!("╔═══════════════════════════════════════╗");
                    println!("║          Available Models             ║");
                    println!("╠═══════════════════════════════════════╣");
                    for model in &models {
                        if let Some(filter) = &args.filter {
                            if !model.to_lowercase().contains(&filter.to_lowercase()) {
                                continue;
                            }
                        }
                        println!("║  {}  ║", model);
                    }
                    println!("╚═══════════════════════════════════════╝");
                }
                shimmy_rehydrated::cli::ListFormat::Json => {
                    println!("{}", serde_json::json!({ "models": models }));
                }
                shimmy_rehydrated::cli::ListFormat::Plain => {
                    for model in models {
                        println!("{}", model);
                    }
                }
            }
        }
        
        Command::Pull(args) => {
            println!("🔮 Pulling model: {}", args.model);
            // Placeholder - would integrate with HF hub
            println!("Note: Model downloading not yet implemented in rehydration");
            println!("Use huggingface-cli or manual download");
        }
        
        Command::Registry(args) => {
            match args.action {
                shimmy_rehydrated::cli::RegistryAction::Add { name, path, lora, template } => {
                    println!("Adding model '{}' at {:?}", name, path);
                    if let Some(l) = lora {
                        println!("  LoRA: {:?}", l);
                    }
                    if let Some(t) = template {
                        println!("  Template: {}", t);
                    }
                    // Placeholder - would persist to config
                }
                shimmy_rehydrated::cli::RegistryAction::Remove { name } => {
                    println!("Removing model '{}'", name);
                }
                shimmy_rehydrated::cli::RegistryAction::List => {
                    let registry = Registry::new();
                    for entry in registry.list() {
                        println!("{}: {:?}", entry.name, entry.base_path);
                    }
                }
            }
        }
        
        Command::Discover(args) => {
            println!("🔮 Running auto-discovery...\n");
            
            let mut discovery = ModelAutoDiscovery::new();
            for path in args.paths {
                discovery.add_search_path(path);
            }
            
            let mut found = discovery.discover_models();
            
            if args.llm_only {
                found = shimmy_rehydrated::auto_discovery::filter_llm_only(found);
            }
            
            match args.format {
                shimmy_rehydrated::cli::ListFormat::Table => {
                    println!("Found {} models:\n", found.len());
                    for (name, model) in &found {
                        println!("  {} ({}) - {} bytes", 
                            name, 
                            model.model_type,
                            model.size_bytes
                        );
                    }
                }
                shimmy_rehydrated::cli::ListFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&found.keys().collect::<Vec<_>>())?);
                }
                shimmy_rehydrated::cli::ListFormat::Plain => {
                    for name in found.keys() {
                        println!("{}", name);
                    }
                }
            }
        }
        
        Command::Info(args) => {
            println!("╔══════════════════════════════════════════════════════════════╗");
            println!("║  Shimmy - Local LLM Inference Server (Rehydrated)            ║");
            println!("╠══════════════════════════════════════════════════════════════╣");
            println!("║  Version: {}                                            ║", env!("CARGO_PKG_VERSION"));
            println!("║  Rehydrated from Sorcery Doctrine spell notation             ║");
            println!("╚══════════════════════════════════════════════════════════════╝");
            
            if args.gpu {
                println!("\nGPU Backends:");
                println!("  • CUDA (NVIDIA) - Check with nvidia-smi");
                println!("  • Vulkan (AMD/NVIDIA) - Check with vulkaninfo");
                println!("  • OpenCL (Any) - Check with clinfo");
                println!("  • Metal (Apple) - macOS only");
                println!("  • MLX (Apple Silicon) - M1/M2/M3 only");
            }
            
            if args.env {
                println!("\nEnvironment Variables:");
                for var in &["SHIMMY_BASE_GGUF", "SHIMMY_MODEL_PATHS", "OLLAMA_MODELS", "HF_TOKEN"] {
                    match std::env::var(var) {
                        Ok(val) => println!("  {} = {}", var, val),
                        Err(_) => println!("  {} = (not set)", var),
                    }
                }
            }
        }
    }
    
    Ok(())
}
