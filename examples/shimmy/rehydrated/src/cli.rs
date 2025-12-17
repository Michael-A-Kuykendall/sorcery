//! CLI - Command Line Interface
//! 
//! Rehydrated from: cli.spell
//! 
//! Clap-based CLI with 8 subcommands.

use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════
// Main CLI Structure
// From cli.spell: @Cli with #[derive(Parser)]
// ═══════════════════════════════════════════════════════════════════

/// Shimmy - Local LLM Inference Server
#[derive(Parser, Debug)]
#[command(name = "shimmy")]
#[command(author, version, about = "Local LLM inference with OpenAI-compatible API")]
pub struct Cli {
    /// Global verbosity level
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

// ═══════════════════════════════════════════════════════════════════
// Subcommands
// From cli.spell: 8 subcommands
// ═══════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the inference server
    Serve(ServeArgs),

    /// Generate text from a prompt (one-shot)
    Generate(GenerateArgs),

    /// Interactive chat session
    Chat(ChatArgs),

    /// List available models
    Models(ModelsArgs),

    /// Download a model from Hugging Face
    Pull(PullArgs),

    /// Manage model registry
    Registry(RegistryArgs),

    /// Run auto-discovery scan
    Discover(DiscoverArgs),

    /// Show configuration and environment
    Info(InfoArgs),
}

// ═══════════════════════════════════════════════════════════════════
// Serve Command
// From cli.spell: serve with host, port, model options
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Model to load at startup
    #[arg(short, long)]
    pub model: Option<String>,

    /// Host to bind to
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// GPU backend to use
    #[arg(long, value_enum)]
    pub gpu: Option<GpuBackendArg>,

    /// Number of GPU layers to offload (-1 = all)
    #[arg(long)]
    pub gpu_layers: Option<i32>,

    /// MoE (Mixture of Experts) CPU offload layers
    #[arg(long)]
    pub moe_cpu_offload: Option<i32>,

    /// Context length override
    #[arg(long)]
    pub ctx_len: Option<usize>,

    /// Number of threads for CPU inference
    #[arg(long)]
    pub threads: Option<i32>,

    /// Enable auto-discovery of models
    #[arg(long, default_value = "true")]
    pub auto_discover: bool,

    /// Disable OpenAI-compatible API endpoints
    #[arg(long)]
    pub no_openai_compat: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Generate Command
// From cli.spell: one-shot text generation
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Model to use
    #[arg(short, long)]
    pub model: String,

    /// Prompt text (or "-" for stdin)
    #[arg(short, long)]
    pub prompt: String,

    /// Maximum tokens to generate
    #[arg(long, default_value = "256")]
    pub max_tokens: usize,

    /// Sampling temperature
    #[arg(long, default_value = "0.7")]
    pub temperature: f32,

    /// Top-p (nucleus) sampling
    #[arg(long)]
    pub top_p: Option<f32>,

    /// Top-k sampling
    #[arg(long)]
    pub top_k: Option<u32>,

    /// GPU backend
    #[arg(long, value_enum)]
    pub gpu: Option<GpuBackendArg>,

    /// GPU layers
    #[arg(long)]
    pub gpu_layers: Option<i32>,

    /// Output format: text, json, markdown
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,
}

// ═══════════════════════════════════════════════════════════════════
// Chat Command
// From cli.spell: interactive REPL chat
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct ChatArgs {
    /// Model to use
    #[arg(short, long)]
    pub model: String,

    /// System prompt
    #[arg(short, long)]
    pub system: Option<String>,

    /// GPU backend
    #[arg(long, value_enum)]
    pub gpu: Option<GpuBackendArg>,

    /// GPU layers
    #[arg(long)]
    pub gpu_layers: Option<i32>,

    /// Temperature
    #[arg(long, default_value = "0.7")]
    pub temperature: f32,
}

// ═══════════════════════════════════════════════════════════════════
// Models Command
// From cli.spell: list available models
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct ModelsArgs {
    /// Show detailed information
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format
    #[arg(long, default_value = "table")]
    pub format: ListFormat,

    /// Filter by name pattern
    #[arg(short, long)]
    pub filter: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════
// Pull Command
// From cli.spell: download models
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct PullArgs {
    /// Model identifier (e.g., "microsoft/phi-3-mini-4k-instruct-gguf")
    pub model: String,

    /// Specific file to download (for multi-file repos)
    #[arg(long)]
    pub file: Option<String>,

    /// Output directory
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Hugging Face token (or set HF_TOKEN env var)
    #[arg(long)]
    pub token: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════
// Registry Command
// From cli.spell: manage model registry
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub action: RegistryAction,
}

#[derive(Subcommand, Debug)]
pub enum RegistryAction {
    /// Add a model to the registry
    Add {
        /// Model name
        #[arg(short, long)]
        name: String,

        /// Path to model file
        #[arg(short, long)]
        path: PathBuf,

        /// Path to LoRA adapter
        #[arg(long)]
        lora: Option<PathBuf>,

        /// Template override
        #[arg(long)]
        template: Option<String>,
    },

    /// Remove a model from the registry
    Remove {
        /// Model name
        name: String,
    },

    /// Show registry entries
    List,
}

// ═══════════════════════════════════════════════════════════════════
// Discover Command
// From cli.spell: run auto-discovery
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct DiscoverArgs {
    /// Additional paths to scan
    #[arg(short, long)]
    pub paths: Vec<PathBuf>,

    /// Only show LLM models (filter out CLIP, SD, etc.)
    #[arg(long, default_value = "true")]
    pub llm_only: bool,

    /// Output format
    #[arg(long, default_value = "table")]
    pub format: ListFormat,
}

// ═══════════════════════════════════════════════════════════════════
// Info Command
// From cli.spell: show configuration
// ═══════════════════════════════════════════════════════════════════

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Show GPU/backend availability
    #[arg(long)]
    pub gpu: bool,

    /// Show environment variables
    #[arg(long)]
    pub env: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Shared Enums
// From cli.spell: value enums for arguments
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum GpuBackendArg {
    Auto,
    Cuda,
    Vulkan,
    OpenCL,
    Metal,
    Mlx,
    Cpu,
}

impl GpuBackendArg {
    pub fn to_engine_backend(&self) -> crate::engine::llama::GpuBackend {
        match self {
            GpuBackendArg::Auto => crate::engine::llama::GpuBackend::Auto,
            GpuBackendArg::Cuda => crate::engine::llama::GpuBackend::Cuda,
            GpuBackendArg::Vulkan => crate::engine::llama::GpuBackend::Vulkan,
            GpuBackendArg::OpenCL => crate::engine::llama::GpuBackend::OpenCL,
            GpuBackendArg::Metal => crate::engine::llama::GpuBackend::Metal,
            GpuBackendArg::Mlx => crate::engine::llama::GpuBackend::Mlx,
            GpuBackendArg::Cpu => crate::engine::llama::GpuBackend::Cpu,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Markdown,
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum ListFormat {
    #[default]
    Table,
    Json,
    Plain,
}

// ═══════════════════════════════════════════════════════════════════
// Parse Helper
// ═══════════════════════════════════════════════════════════════════

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
