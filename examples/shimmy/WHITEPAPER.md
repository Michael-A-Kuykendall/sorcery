# 📜 Shimmy Dehydration Whitepaper

**Authorship note:** This dehydration writeup was produced by **Opus Claude 4.5**.

> **Architectural Extraction and Notation for System-Scale Software**

## Abstract

This whitepaper documents the complete dehydration of **Shimmy**, a 4.8MB Rust-based OpenAI API-compatible inference server, into Sorcery spell notation. We demonstrate that the 9-glyph grammar is sufficient to capture complex system architecture including async traits, FFI bindings, multi-backend polymorphism, and API compatibility layers.

---

## 1. System Overview

### 1.1 What Shimmy Does

Shimmy provides local LLM inference with drop-in OpenAI API compatibility:

```
┌─────────────────────────────────────────────────────────────────┐
│                           Shimmy                                │
│                                                                 │
│  ┌─────────┐    ┌──────────┐    ┌────────────────────────────┐ │
│  │   CLI   │───▶│  Server  │───▶│     API Handlers           │ │
│  └─────────┘    └──────────┘    │  • /api/generate           │ │
│                      │          │  • /v1/chat/completions    │ │
│                      ▼          │  • /v1/models              │ │
│  ┌──────────────────────────┐   │  • /ws/generate            │ │
│  │      Model Registry      │   └────────────────────────────┘ │
│  │  ┌────────┬────────────┐ │              │                   │
│  │  │ Manual │ Discovered │ │              ▼                   │
│  │  └────────┴────────────┘ │   ┌────────────────────────────┐ │
│  └──────────────────────────┘   │    Inference Engines       │ │
│              │                  │  • LlamaEngine (llama.cpp) │ │
│              ▼                  │  • HuggingFaceEngine       │ │
│  ┌──────────────────────────┐   │  • MLXEngine (Apple)       │ │
│  │    Auto Discovery        │   │  • SafeTensorsEngine       │ │
│  │  ~/.cache/huggingface    │   └────────────────────────────┘ │
│  │  ~/.ollama/models        │                                  │
│  │  ~/.lmstudio/models      │                                  │
│  └──────────────────────────┘                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Technical Stack

| Layer | Technology |
|-------|------------|
| Language | Rust 2021 |
| Async Runtime | Tokio |
| HTTP Framework | Axum |
| CLI Parsing | Clap (derive macros) |
| Inference Backend | llama.cpp (via FFI) |
| Serialization | Serde JSON |
| Streaming | SSE / WebSockets |

---

## 2. Dehydration Methodology

### 2.1 Module Identification

We identified **10 core modules** that constitute Shimmy's architecture:

1. **cli** - Command-line interface with Clap
2. **model_registry** - Dual-source model storage
3. **auto_discovery** - Filesystem scanning for models
4. **engine** - Core trait definitions
5. **engine/llama** - llama.cpp backend
6. **engine/adapter** - Multi-backend routing
7. **server** - HTTP server and routing
8. **api** - Native API handlers
9. **openai_compat** - OpenAI compatibility layer
10. **templates** - Prompt formatting

### 2.2 Glyph Application

Each glyph maps to specific Rust constructs:

| Glyph | Rust Construct | Shimmy Examples |
|-------|---------------|-----------------|
| `#` | struct field | `#name -> String` |
| `^` | impl trait / derive | `^ Send + Sync` |
| `@` | struct / enum | `@ModelEntry`, `@GpuBackend` |
| `:` | method / variant | `:load()`, `:Cuda` |
| `!` | invariant assertion | `! manual takes priority` |
| `~` | assumption | `~ rust_semantics` |
| `->` | return type / flow | `-> Result<Box<dyn @LoadedModel>>` |
| `?` | Option type | `?PathBuf` |

### 2.3 Assumptions (`~`) Catalog

Each spell begins with semantic assumptions:

```
~ rust_semantics           # Rust language semantics apply
~ async_trait_pattern      # async_trait crate pattern
~ tokio_runtime           # Tokio async executor
~ hashmap_storage         # std::collections::HashMap
~ pathbuf_paths           # std::path::PathBuf
~ axum_framework          # Axum web framework
~ ffi_bindings            # Foreign Function Interface
~ feature_flags           # Cargo feature-based compilation
```

These establish the **semantic context** without repeating language fundamentals.

---

## 3. Architectural Patterns Captured

### 3.1 Trait Object Polymorphism

The core abstraction uses trait objects for runtime dispatch:

```
@InferenceEngine
  ^ Send + Sync
  ^ async_trait

  :load(spec: &@ModelSpec) -> Result<Box<dyn @LoadedModel>>
```

**Captured in**: `engine.spell`

This pattern allows hot-swapping backends:
- `LlamaEngine` for GGUF files
- `HuggingFaceEngine` for PyTorch models
- `MLXEngine` for Apple Silicon
- `SafeTensorsEngine` for pure Rust inference

### 3.2 Dual-Source Registry Pattern

The model registry maintains two sources with priority ordering:

```
@Registry
  #inner -> HashMap<String, @ModelEntry>           # manual, HIGH priority
  #discovered_models -> HashMap<String, @DiscoveredModel>  # auto, LOW priority

  :to_spec(name: &str) -> ?@ModelSpec
    ! manual entries take priority over discovered
```

**Captured in**: `model_registry.spell`

### 3.3 Backend Adapter Pattern

The adapter routes to appropriate engines based on file type:

```
@InferenceEngineAdapter
  ^ @InferenceEngine

  #llama_engine -> ?@LlamaEngine
  #huggingface_engine -> ?@HuggingFaceEngine
  #mlx_engine -> ?@MLXEngine
  #safetensors_engine -> ?@SafeTensorsEngine

  :select_backend(spec: &@ModelSpec) -> @BackendChoice
    # .gguf -> Llama
    # .safetensors -> SafeTensors or HuggingFace
    # .bin -> HuggingFace
    # .npz -> MLX
```

**Captured in**: `engine_adapter.spell`

### 3.4 SSE Streaming Pattern

OpenAI-compatible streaming uses Server-Sent Events:

```
:chat_completions(State(state), Json(req)) -> impl IntoResponse
  # if req.stream == true:
  #   a. create unbounded channel
  #   b. spawn generation task with token callback
  #   c. send initial chunk: { delta: { role: "assistant" } }
  #   d. for each token: send chunk with content
  #   e. send final chunk: { finish_reason: "stop" }
  #   f. send "[DONE]"
```

**Captured in**: `openai_compat.spell`

### 3.5 Template Family Pattern

Prompt formatting varies by model family:

```
@TemplateFamily
  :ChatML       # <|im_start|>user\n...<|im_end|>
  :Llama3       # <|start_header_id|>user<|end_header_id|>
  :OpenChat     # GPT4 Correct User: ...

  :render(system, history, user_input) -> String
  :stop_tokens() -> Vec<String>
```

**Captured in**: `templates.spell`

---

## 4. Inter-Module Dependencies

```
                    ┌─────────┐
                    │   cli   │
                    └────┬────┘
                         │
           ┌─────────────┼─────────────┐
           ▼             ▼             ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐
    │  server  │  │ registry │  │ discovery│
    └────┬─────┘  └────┬─────┘  └──────────┘
         │             │
    ┌────┴────────────┬┘
    ▼                 ▼
┌───────┐      ┌──────────────┐
│  api  │      │    engine    │
└───┬───┘      │   (traits)   │
    │          └──────┬───────┘
    │                 │
    │    ┌────────────┼────────────┐
    │    ▼            ▼            ▼
    │ ┌──────┐  ┌─────────┐  ┌─────────┐
    │ │llama │  │  hf     │  │   mlx   │
    │ └──────┘  └─────────┘  └─────────┘
    │         \      │      /
    │          ▼     ▼     ▼
    │        ┌─────────────┐
    │        │   adapter   │
    │        └─────────────┘
    │
    ├──────────────┬────────────────┐
    ▼              ▼                ▼
┌───────────┐ ┌────────────┐ ┌───────────┐
│ openai_   │ │ anthropic_ │ │ templates │
│ compat    │ │ compat     │ │           │
└───────────┘ └────────────┘ └───────────┘
```

---

## 5. Invariants and Guarantees

The spells capture critical system invariants:

### 5.1 Priority Ordering
```
! manual entries take priority over discovered
```
Manual model registrations always override auto-discovered models with the same name.

### 5.2 Thread Safety
```
! implementors must be Send + Sync
! Mutex ensures single inference at a time
```
All engine implementations must be thread-safe for use in async contexts.

### 5.3 Mutual Exclusion
```
! conflicts_with(cpu_moe, n_cpu_moe)
```
CLI flags `--cpu-moe` and `--n-cpu-moe` are mutually exclusive.

### 5.4 Streaming Semantics
```
! on_token callback enables streaming
! streaming uses Server-Sent Events (SSE)
```
Token callbacks must be honored for streaming responses.

---

## 6. Compression Analysis

| Metric | Original | Dehydrated | Ratio |
|--------|----------|------------|-------|
| Rust code | ~5,000 lines | - | - |
| Spell notation | - | ~800 lines | - |
| **Compression** | - | - | **6.25x** |

The spells achieve **6.25x compression** while retaining:
- All data structures
- All trait hierarchies
- All method signatures
- Inter-module relationships
- Invariants and assumptions
- Flow control patterns

---

## 7. What's NOT Captured

The spells deliberately omit:

1. **Implementation details** - Exact algorithms, loop bodies
2. **Error handling minutiae** - Specific error messages
3. **Test code** - Unit tests and integration tests
4. **Cargo.toml** - Dependencies and features (partially in `~`)
5. **Comments in code** - Inline documentation
6. **Performance optimizations** - Specific memory layouts

These are **derivable** from the architectural spec during rehydration.

---

## 8. Rehydration Instructions

To reconstruct Shimmy from these spells:

### 8.1 Project Setup
```bash
cargo new shimmy
cd shimmy
```

### 8.2 Dependencies (infer from `~` assumptions)
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
tracing = "0.1"

[dependencies.shimmy_llama_cpp_2]
version = "0.1"
optional = true

[features]
default = ["llama"]
llama = ["shimmy_llama_cpp_2"]
llama-cuda = ["llama"]
llama-vulkan = ["llama"]
huggingface = []
mlx = []
gpu = ["llama-cuda", "llama-vulkan"]
```

### 8.3 Module Creation Order
1. `src/engine/mod.rs` - Traits first
2. `src/model_registry.rs` - Data structures
3. `src/auto_discovery.rs` - Discovery
4. `src/engine/llama.rs` - Primary backend
5. `src/engine/adapter.rs` - Routing
6. `src/templates.rs` - Prompts
7. `src/api.rs` - Handlers
8. `src/openai_compat.rs` - Compatibility
9. `src/server.rs` - HTTP
10. `src/cli.rs` - Entry point
11. `src/main.rs` - Wire it together

### 8.4 Success Criteria
The rehydrated code should:
- [ ] Compile without errors
- [ ] Pass `shimmy --help`
- [ ] Pass `shimmy serve --bind 127.0.0.1:11435`
- [ ] Respond to `GET /health`
- [ ] Respond to `GET /v1/models`
- [ ] Accept models in `./models/` directory
- [ ] Auto-discover from common paths

---

## 9. Conclusion

This whitepaper demonstrates that **Sorcery notation scales to system-level architecture**. The 9-glyph grammar successfully captured:

- **Polymorphic trait hierarchies** (InferenceEngine, LoadedModel)
- **Async patterns** (tokio, SSE streaming)
- **FFI abstractions** (llama.cpp bindings)
- **Multi-backend routing** (adapter pattern)
- **API compatibility layers** (OpenAI, Anthropic)
- **Discovery systems** (multi-path filesystem scanning)

The spells serve as a **complete architectural handoff specification**, enabling reconstruction by any competent Rust developer (or AI) without access to the original source.

---

*Dehydrated by the Sorcery Doctrine*
*The Grammar is Complete. The Incantations are Infinite.*
