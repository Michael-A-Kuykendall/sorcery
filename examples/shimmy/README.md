# 🏰 Shimmy Case Study

> **The Grand Experiment: Can Sorcery Spells Describe an Entire System?**

## What is Shimmy?

**Shimmy** is a 4.8MB single-binary OpenAI API-compatible local inference server written in Rust. It supports:

- **llama.cpp** backend with GGUF models
- **Multiple GPU backends**: CUDA, Vulkan, OpenCL, Metal (MLX)
- **MoE CPU Offloading** for large mixture-of-experts models
- **Auto-discovery** from HuggingFace, Ollama, and LM Studio caches
- **OpenAI API compatibility** (drop-in replacement)
- **Anthropic API compatibility**
- **WebSocket streaming**
- **Response caching and observability**

It is a **production-grade system**, not a toy library.

## The Challenge

Previous Sorcery examples (mitt, dlv, clsx, once, ms, fast-deep-equal) are **small utilities** – typically 50-200 lines of code. They prove the grammar works for compression, but leave a question unanswered:

> **Can Sorcery scale to describe *entire systems*?**

Shimmy provides the answer. It consists of:

- **15+ modules** (CLI, server, API, multiple engine backends)
- **~5,000+ lines of Rust code**
- **Complex inter-module dependencies**
- **Trait-based polymorphism** (InferenceEngine, LoadedModel)
- **Async/await runtime** (Tokio)
- **FFI bindings** (llama.cpp)
- **Multiple API compatibility layers**

## The Dehydration

We decomposed Shimmy into **10 spell files**:

| Spell | Module | Purpose |
|-------|--------|---------|
| [cli.spell](spells/cli.spell) | Command Line | Clap-based CLI with 8 subcommands |
| [model_registry.spell](spells/model_registry.spell) | Model Hub | Dual-source registry (manual + discovered) |
| [auto_discovery.spell](spells/auto_discovery.spell) | Discovery | Filesystem scanning for GGUF/SafeTensors |
| [engine.spell](spells/engine.spell) | Abstraction | InferenceEngine/LoadedModel traits |
| [engine_llama.spell](spells/engine_llama.spell) | llama.cpp | GPU backends, MoE offloading |
| [engine_adapter.spell](spells/engine_adapter.spell) | Routing | Multi-backend adapter pattern |
| [server.spell](spells/server.spell) | HTTP | Axum routes, CORS, health checks |
| [api.spell](spells/api.spell) | Native API | /api/generate, WebSocket streaming |
| [openai_compat.spell](spells/openai_compat.spell) | OpenAI | /v1/chat/completions, SSE streaming |
| [templates.spell](spells/templates.spell) | Prompts | ChatML, Llama3, OpenChat formats |

### Total: ~800 lines of spell notation

This captures the **architectural essence** of a 5000+ line Rust project.

## What the Spells Capture

### 1. **Data Structures** (`@` Components)
```
@ModelEntry
  #name -> String
  #base_path -> PathBuf
  #lora_path -> ?PathBuf
  #template -> ?String
```

### 2. **Trait Hierarchies** (`^` Inheritance + `:` Methods)
```
@InferenceEngine
  ^ Send + Sync
  ^ async_trait

  :load(spec: &@ModelSpec) -> Result<Box<dyn @LoadedModel>>
```

### 3. **Enums as Variants** (`:Variant`)
```
@GpuBackend
  :Cpu
  :Cuda
  :Vulkan
  :OpenCL
  :Metal
```

### 4. **Obligations** (`$`)
```
$ require: manual entries take priority over discovered -> test: manual_priority
$ require: Mutex ensures single inference at a time -> test: single_inference
```

### 5. **Assumptions** (`~`)
```
~ rust_semantics
~ async_trait_pattern
~ tokio_runtime
```

### 6. **Flow Control and Architecture** (ASCII diagrams)
```
# ┌─────────────────────────────────────────────────────────┐
# │                      Registry                          │
# ├────────────────────────┬────────────────────────────────┤
# │  inner (manual)        │  discovered_models (auto)     │
# │  PRIORITY: HIGH        │  PRIORITY: LOW                │
# └────────────────────────┴────────────────────────────────┘
```

## The Strict Separation

**Phase 1: DEHYDRATION** (Complete)
- Analyzed Shimmy's GitHub repository
- Extracted architectural patterns
- Wrote spell files capturing intent
- **No code was written** – only notation

**Phase 2: DOCUMENTATION** (This file + WHITEPAPER)
- Explain what was captured
- Document the inter-module relationships
- Provide rehydration guidance

**Phase 3: REHYDRATION** (Next)
- An AI (or human) will attempt to reconstruct Shimmy
- **Using only the spell files**
- **Without access to the original source**
- Success = working Rust code that compiles and runs

## Why This Matters

If rehydration succeeds, it proves:

1. **Spells are sufficient** for architectural handoff
2. **The grammar is complete** – no "missing glyphs"
3. **Compression is achievable** – 800 lines → 5000+ lines
4. **AI can understand architecture** through declarative notation

## The Spell Manifest

```
spells/
├── cli.spell              # CLI argument parsing
├── model_registry.spell   # Model storage and lookup
├── auto_discovery.spell   # Filesystem scanning
├── engine.spell           # Core traits
├── engine_llama.spell     # llama.cpp backend
├── engine_adapter.spell   # Multi-backend routing
├── server.spell           # HTTP server
├── api.spell              # Native API handlers
├── openai_compat.spell    # OpenAI compatibility
└── templates.spell        # Prompt formatting
```

## Reading Order for Rehydration

1. **engine.spell** – Core traits first
2. **model_registry.spell** – Data structures
3. **auto_discovery.spell** – Discovery logic
4. **engine_llama.spell** – Primary backend
5. **engine_adapter.spell** – Backend routing
6. **templates.spell** – Prompt formatting
7. **api.spell** – API handlers
8. **openai_compat.spell** – Compatibility layer
9. **server.spell** – HTTP routing
10. **cli.spell** – Entry point

---

**Next Step:** Read [WHITEPAPER.md](WHITEPAPER.md) for the full dehydration analysis, then attempt rehydration in `rehydrated/`.
