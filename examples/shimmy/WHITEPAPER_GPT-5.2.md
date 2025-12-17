# Sorcery Case Study: Shimmy (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only the Sorcery spell files under [examples/shimmy/spells](examples/shimmy/spells), I rehydrated a Rust 2021 Tokio/Axum inference server into a fresh Cargo project at [examples/shimmy/rehydrated2](examples/shimmy/rehydrated2).

Scope for this session was “rehydrate the architecture and compile,” not “match upstream feature-for-feature.” I did not consult any upstream Shimmy source while implementing.

**Result:** The rehydrated project compiles (`cargo check`), implements the spell-described architecture (registry + discovery + templates + OpenAI-compatible routes), and intentionally uses placeholders where the spells allowed “backend details” to remain abstract.

---

## The Subject

Shimmy (as described by the spells) is a local LLM inference server with:
- a CLI surface
- a model registry combining manual entries and auto-discovered models
- backend selection via an engine adapter
- prompt templates by “family”
- HTTP routes for a native API and OpenAI-compatible endpoints (including streaming)

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [cli.spell](examples/shimmy/spells/cli.spell) | CLI structure + commands |
| [model_registry.spell](examples/shimmy/spells/model_registry.spell) | Manual + discovered registry patterns |
| [auto_discovery.spell](examples/shimmy/spells/auto_discovery.spell) | Filesystem model discovery |
| [engine.spell](examples/shimmy/spells/engine.spell) | Core traits + options |
| [engine_llama.spell](examples/shimmy/spells/engine_llama.spell) | Llama backend contract |
| [engine_adapter.spell](examples/shimmy/spells/engine_adapter.spell) | Multi-backend routing |
| [templates.spell](examples/shimmy/spells/templates.spell) | Prompt formatting families |
| [server.spell](examples/shimmy/spells/server.spell) | Axum router wiring |
| [api.spell](examples/shimmy/spells/api.spell) | Native API handlers |
| [openai_compat.spell](examples/shimmy/spells/openai_compat.spell) | `/v1/*` compatibility + streaming |

---

## Rehydrated2 Structure

Primary code locations:
- Cargo project: [examples/shimmy/rehydrated2/Cargo.toml](examples/shimmy/rehydrated2/Cargo.toml)
- Entrypoint: [examples/shimmy/rehydrated2/src/main.rs](examples/shimmy/rehydrated2/src/main.rs)
- Library module root: [examples/shimmy/rehydrated2/src/lib.rs](examples/shimmy/rehydrated2/src/lib.rs)
- HTTP server wiring: [examples/shimmy/rehydrated2/src/server.rs](examples/shimmy/rehydrated2/src/server.rs)
- Native API: [examples/shimmy/rehydrated2/src/api.rs](examples/shimmy/rehydrated2/src/api.rs)
- OpenAI compat: [examples/shimmy/rehydrated2/src/openai_compat.rs](examples/shimmy/rehydrated2/src/openai_compat.rs)
- Model registry: [examples/shimmy/rehydrated2/src/model_registry.rs](examples/shimmy/rehydrated2/src/model_registry.rs)
- Discovery: [examples/shimmy/rehydrated2/src/auto_discovery.rs](examples/shimmy/rehydrated2/src/auto_discovery.rs)
- Templates: [examples/shimmy/rehydrated2/src/templates.rs](examples/shimmy/rehydrated2/src/templates.rs)
- Engine layer: [examples/shimmy/rehydrated2/src/engine](examples/shimmy/rehydrated2/src/engine)

---

## Comparison (Spell Intent → Rehydrated2)

| Area | Spell intent | Rehydrated2 | Verdict |
|------|-------------|-------------|---------|
| CLI surface | Clap-driven command enum | Implemented via `clap` derive | ✅ |
| Dual-source registry | manual priority over discovered | Implemented (manual first lookup) | ✅ |
| Auto-discovery | scan known dirs + env overrides | Implemented with bounded walk depth | ✅ |
| Engine abstraction | trait-based `InferenceEngine` + `LoadedModel` | Implemented in engine module | ✅ |
| Adapter routing | choose backend by model kind | Implemented (ext-based selection) | ✅ |
| Llama backend | supports generate + streaming callback | Implemented as placeholder engine with streaming callback | ⚠️ Placeholder backend |
| Templates | ChatML/Llama/OpenChat families + stop tokens | Implemented, with simple inference by model name | ✅ |
| Native API | `/api/*` endpoints + SSE streaming for generate | Implemented | ✅ |
| OpenAI compat | `/v1/chat/completions` + streaming | Implemented (SSE) | ✅ |
| “No wild extras” | avoid features not in spells | Stuck to spell-described surfaces | ✅ |

---

## Notes From This Rehydration Session

- The spell set is architecture-heavy; for the backend engine, the spells express contracts and boundaries rather than concrete llama.cpp bindings. I kept the llama engine intentionally minimal while still satisfying async + streaming shapes.
- While bringing the Rust project to a compiling state, I fixed several “rehydration-typical” errors (ownership in async closures, Axum middleware signature mismatches, and an async `Send` issue caused by holding a `MutexGuard` across an await). Those fixes are reflected in the current `rehydrated2` code.

---

## Reproduce

From the repo root:
- `cd examples/shimmy/rehydrated2`
- `cargo check`

This validates that the rehydration is internally consistent and compiles as a Rust project.
