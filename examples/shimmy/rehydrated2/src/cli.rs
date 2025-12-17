use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "shimmy")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "single-binary GGUF + LoRA server")]
pub struct Cli {
	#[command(subcommand)]
	pub cmd: Command,

	/// Semicolon-separated model directories
	#[arg(long = "model-dirs")]
	pub model_dirs: Option<String>,

	/// GPU backend: auto|cpu|cuda|vulkan|opencl
	#[arg(long = "gpu-backend")]
	pub gpu_backend: Option<String>,

	/// Offload ALL MoE experts to CPU
	#[arg(long = "cpu-moe", conflicts_with = "n_cpu_moe")]
	pub cpu_moe: bool,

	/// Offload first N layers of experts to CPU
	#[arg(long = "n-cpu-moe")]
	pub n_cpu_moe: Option<usize>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	Serve {
		#[arg(long, default_value = "auto")]
		bind: String,
		#[arg(long)]
		model_path: Option<String>,
	},
	List {
		#[arg(short, long)]
		short: bool,
	},
	Discover {
		#[arg(long)]
		llm_only: bool,
	},
	Probe {
		name: String,
	},
	Bench {
		name: String,
		#[arg(long, default_value_t = 64)]
		max_tokens: usize,
	},
	Generate {
		name: String,
		#[arg(long)]
		prompt: String,
		#[arg(long, default_value_t = 64)]
		max_tokens: usize,
	},
	GpuInfo,
	Init {
		template: String,
		#[arg(long, default_value = ".")]
		output: String,
		#[arg(long)]
		name: Option<String>,
	},
}
