# Rehydration v1 vs v2 (and Shimmy vs upstream)

This document compares two independent rehydrations produced from the Sorcery Doctrine “spells”:

- **v1**: `examples/*/rehydrated/…`
- **v2**: `examples/*/rehydrated2/…` (produced in a separate session)

The goal is not byte-for-byte identity; it’s to identify:

1) **Consistent omissions** (likely spell gaps), vs
2) **Variable implementation choices** (model/session variance), and
3) Whether either rehydration is **closer to upstream** in the Shimmy case.

---

## TypeScript examples (v1 vs v2)

### High-level finding
Across all TS examples, v2 tends to be **shorter** and often shifts toward **default-export-first** APIs. v1 tends to include **extra helper exports** and/or more internal scaffolding.

That difference is important: it means the spells are currently underspecified around **module export shape** and **public API surface**.

### Diffstat summary
(Counts are `insertions/deletions` for `v1 -> v2`.)

| Example | v1 file | v2 file | Diffstat (v1→v2) |
|---|---|---|---|
| clsx | `examples/clsx/rehydrated/clsx.ts` | `examples/clsx/rehydrated2/clsx.ts` | 50 / 83 |
| dlv | `examples/dlv/rehydrated/dlv.ts` | `examples/dlv/rehydrated2/dlv.ts` | 31 / 43 |
| once | `examples/once/rehydrated/once.ts` | `examples/once/rehydrated2/once.ts` | 114 / 117 |
| ms | `examples/ms/rehydrated/ms.ts` | `examples/ms/rehydrated2/ms.ts` | 106 / 236 |
| fast-deep-equal | `examples/fast-deep-equal/rehydrated/equal.ts` | `examples/fast-deep-equal/rehydrated2/equal.ts` | 126 / 197 |
| mitt | `examples/mitt/rehydrated/mitt.rehydrated.ts` | `examples/mitt/rehydrated2/mitt.ts` | 65 / 137 |

### API surface differences (exports)

| Example | v1 exports (observed) | v2 exports (observed) | Notes |
|---|---|---|---|
| clsx | named `clsx`, plus `default` | `default` only | Export shape diverges.
| dlv | `default` | `default` + exported `Path` type | v2 adds a public type alias.
| once | named `once`, named `onceStrict` | `default` once + named `onceStrict` | Default export differs.
| ms | `parse`, `format`, overloads + `default` | overloads + `default` only | v1 exposes internals; v2 hides them.
| fast-deep-equal | named `equal`, named `equalES6` | `default` equal + named `es6Equal` | Name/export differences.
| mitt | `default` mitt + types | `default` mitt + types (slightly different generics/types) | Largely aligned; type-level differences.

---

## Shimmy (v1 vs v2 vs upstream)

### Repository used for upstream
Upstream Shimmy was cloned from:

- https://github.com/Michael-A-Kuykendall/shimmy

### Source-module coverage (presence)
Considering upstream `src/**/*.rs` excluding `src/bin/**` and `src/tests/**`:

- Upstream modules considered: **36**
- Present in v1: **12**
- Present in v2: **13**

**Only in v2 (not in v1):**
- `src/anthropic_compat.rs`

**Missing in BOTH (examples):**
- `src/metrics.rs`, `src/observability/mod.rs`
- `src/cache/*`
- `src/model_manager.rs`, `src/workflow.rs`, `src/preloading.rs`
- `src/engine/huggingface.rs`, `src/engine/mlx.rs`, `src/engine/universal.rs`, `src/engine/safetensors_native.rs`
- `src/util/diag.rs`, `src/util/memory.rs`

Interpretation: these are **consistent omissions**, so they’re strong candidates for **spell coverage gaps**.

### v1 vs v2 (within rehydrations)
Directory diffstat (v1 src → v2 src):

- 13 files changed, **1906 insertions**, **2498 deletions**
- New in v2: `src/anthropic_compat.rs`

Qualitative read:

- v2 is generally **more compact** (more deletions overall), and adds a **minimal** Anthropic compatibility module.
- v1 tends to have **more “invented” glue** (more novel lines vs upstream), while v2 tends to have **less novel code**.

### Closeness-to-upstream (rough quant)
Using a simple “diff size” proxy on the overlapping major files:

- v1 vs upstream: **10062** total changed lines (2753 insertions + 7309 deletions)
- v2 vs upstream: **9823** total changed lines (2161 insertions + 7662 deletions)

Interpretation:

- v2 is **slightly closer overall** by this metric.
- However, v2 is also missing a bit more upstream content (higher deletions), while v1 adds more novel code (higher insertions).

### Key per-file signals
(Insertions = rehydration-only lines; deletions = upstream-only lines.)

- `src/cli.rs`: v2 adds far fewer novel lines than v1.
- `src/openai_compat.rs`: v2 adds fewer novel lines than v1.
- `src/anthropic_compat.rs`: v2 exists but is heavily truncated relative to upstream.

### Noise note: build artifacts
Shimmy v2 includes `examples/shimmy/rehydrated2/target/**` artifacts from compilation. These should be excluded from any meaningful comparison.

---

## What this comparison says about the method

- The spells are doing well at reconstructing **core architecture** (server/API/OpenAI compat/engine abstractions).
- The spells are currently weak at pinning down:
  - **Export surfaces** for TS libraries (default vs named exports; which helpers are public)
  - Shimmy’s **production subsystems** (metrics/observability/cache/workflow/etc.)

If you want, the next step is to take the consistent omissions list and write a **spell addendum** that encodes those missing subsystems explicitly.
