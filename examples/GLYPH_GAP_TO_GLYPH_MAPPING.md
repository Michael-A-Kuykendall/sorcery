# Gap → Glyph Mapping (Are we missing glyphs, or missing spell logic?)

**Date:** 2025-12-17  
**Scope:** All observed gaps from:
- Shimmy upstream vs rehydrations (v1/v2)
- TS example rehydrated vs rehydrated2 differences (public export surface variance)

This document answers a very specific question:

> Do the observed gaps imply missing glyphs, or were they logical/plan construction failures where the existing glyphs already had enough expressive power?

## TL;DR

- The large majority of gaps are **not missing-glyph problems**.
- They are **spell completeness problems**:
  - missing entities (`@`) and missing explicit contracts (`:`)
  - missing “exact inventory / do-not-invent” invariants (`!`) and exclusions (`-`)
  - missing explicit dependency/flow edges (`>`) that would force specific modules to exist
  - missing assumptions (`~`) about platform-specific behavior
- The doctrine can likely avoid most divergence by adding a casting rule: **if a set is meant to be exact (commands, endpoints, exports), it must be declared as exact via `!` and protected via `-`**.

A new glyph is **not strictly required** for any gap listed here.

---

## Glyph cheat-sheet (from doctrine)

| Glyph | Meaning | What it should have prevented here |
|---|---|---|
| `@` | Entity / component | Missing subsystems should show up as explicit entities |
| `:` | Contract (input → output / signature) | Missing endpoints/commands should appear as explicit signatures |
| `!` | Guarantee / invariant | “Exact inventory” and “no invention” can be guarantees |
| `~` | Assumption | Platform gating and “depends on OS conventions” can be assumptions |
| `-` | Exclusion | “No extra commands/endpoints/exports” is an exclusion |
| `>` | Dependency / flow | Forces existence of required modules and data paths |
| `?` | Open question | Can block invocation if inventory is uncertain |

---

## Classification scheme used in this report

Each gap is tagged as one of:

- **S1 — Spell Omission:** The spell never mentioned the subsystem; the rehydration cannot invent it.
- **S2 — Underspecified Constraint:** The spell implied something, but did not lock it down (common with inventories, names, and surfaces).
- **S3 — Boundary/Scope Choice:** The spell intentionally targets architecture and skipped production hardening. Still expressible, but may be out-of-scope.
- **S4 — Implementation Detail (but still expressible):** Binary formats, regex exactness, etc. These can be captured using existing glyphs, but require deliberate casting.

---

## Shimmy: observed gaps mapped to glyphs

Source of gaps:
- Shimmy upstream vs rehydrated (forensics + diff analysis)
- Upstream module presence check (v1 had 12/36 modules, v2 had 13/36 excluding bins/tests)

### 1) CLI command inventory mismatches (missing + invented commands)

**Observed:** Rehydration produced a plausible CLI, but the command set differed from upstream.

- **Class:** S2 (Underspecified constraint)
- **Primary glyphs that should solve it:** `!` + `-` + `:`

**What went wrong logically:**
- The spell captured “there is a CLI with commands” (shape), but did not specify that the set was **exact**, allowing the rehydrator to “complete the design”.

**How to express it with current glyphs (spell upgrade pattern):**

```glyph
#Spell: CLI
^ Intent: provide the exact operational interface for Shimmy

@CLI
  ! command_inventory_exact
  - invented_commands

@Command:Serve
  : args -> exit_code

@Command:List
  : args -> exit_code

@Command:Discover
  : args -> exit_code

@Command:Probe
  : args -> exit_code

@Command:Bench
  : args -> exit_code

@Command:Init
  : args -> exit_code

@Command:GpuInfo
  : args -> exit_code

@Command:Generate
  : args -> exit_code
```

If you truly don’t want to bind names, use `?` to block:

```glyph
? command inventory is exact vs representative
```

---

### 2) Missing Ollama manifest/blob parsing

**Observed:** upstream parses Ollama manifests and performs GGUF blob detection; rehydration lacked this.

- **Class:** S4 (Implementation detail, but expressible)
- **Primary glyphs:** `@` + `:` + `!` + `>`

**What went wrong logically:**
- “Supports Ollama discovery” was cast as a path search, not as a protocol contract.

**How to express it with current glyphs:**

```glyph
#Spell: OllamaDiscovery
^ Intent: discover Ollama models correctly and safely

@OllamaManifest
  #schema_version -> i32
  #media_type -> String
  #config -> @OllamaConfig
  #layers -> Vec<@OllamaLayer>

@DiscoverOllama
  : root_dir -> Result<Vec<@DiscoveredModel>>
  ! manifest_parsed
  ! gguf_blob_validated
  - guess_by_extension_only
  > @OllamaManifest
```

---

### 3) Missing observability (/metrics, /diag) + GPU detection helpers

**Observed:** upstream exposes metrics and diagnostics; rehydration only kept health.

- **Class:** S1/S3 (Not in spells; often treated as “infra”) 
- **Primary glyphs:** `@` + `:` + `!` + `-` + `>`

**What went wrong logically:**
- Observability was not declared as part of the architecture intent, so it became optional.

**Spell upgrade (still using existing glyphs):**

```glyph
#Spell: Observability
^ Intent: provide minimal production observability for Shimmy

@Metrics
  : http_request -> prometheus_text
  ! safe_under_load
  - blocks_inference

@Diag
  : http_request -> json
  ! no_secrets
  - filesystem_dump

@Server
  : routes -> router
  > @Metrics
  > @Diag
```

---

### 4) Missing Anthropic compatibility (v1 missing; v2 has a minimal module)

**Observed:** upstream supports `/v1/messages`; v1 omitted, v2 included a reduced version.

- **Class:** S1 → becomes S2 variance (because it was not encoded as required)
- **Primary glyphs:** `@` + `:` + `!` + `>`

**Spell upgrade:**

```glyph
#Spell: AnthropicCompat
^ Intent: expose Anthropic-compatible messages endpoint

@AnthropicAPI
  : POST /v1/messages -> Response
  ! request_schema_supported
  ! streaming_supported_or_explicitly_rejected

@Server
  > @AnthropicAPI
```

Note: You can encode “must exist” without encoding every field; the key is the dependency edge (`>`) and the endpoint contract (`:`).

---

### 5) Missing deployment templates (templates.rs did double duty upstream)

**Observed:** rehydration treated templates as prompt formatting only; upstream also uses templates for deploy scaffolding.

- **Class:** S2 (Spell boundary mismatch)
- **Primary glyphs:** `#` + `^` + `@` + `:`

**What went wrong logically:**
- The spell’s intent was “prompt templates”, but upstream file name suggested broader scope. This isn’t a glyph gap; it’s a **spell slicing** gap.

**Spell upgrade:** split into two spells:

```glyph
#Spell: PromptTemplates
^ Intent: render prompts in known chat formats

@PromptTemplate
  : messages -> prompt
  ! stable_format

#Spell: DeployTemplates
^ Intent: generate deployment scaffolding for common platforms

@DeployTemplate
  : config -> files
  ! reproducible
  - secrets_in_output
```

---

### 6) Missing PPT invariants / runtime contract assertions

**Observed:** upstream uses invariant checks; rehydration omitted them.

- **Class:** S3 (Quality layer) but expressible
- **Primary glyphs:** `!` (guarantees) + `>` (flow to enforcement)

**What went wrong logically:**
- Guarantees existed in prose in places, but there was no spell/entity representing the **enforcement mechanism**.

**Spell upgrade pattern:**

```glyph
#Spell: Invariants
^ Intent: enforce critical runtime invariants at subsystem boundaries

@ShimmyInvariants
  : discovered_models -> ok
  ! asserts_discovery_valid
  ! asserts_backend_selection_valid

@AutoDiscovery
  > @ShimmyInvariants
```

---

### 7) Windows-specific path scanning (drive-letter probing)

**Observed:** upstream has explicit Windows drive scanning; rehydration used generic HOME/USERPROFILE.

- **Class:** S2/S4 (Platform-specific behavior underspecified)
- **Primary glyphs:** `~` + `!` + `@` + `:`

**Key point:** you don’t need conditional-compilation syntax to represent conditional behavior. You can cast a platform spell whose assumptions explicitly gate it.

```glyph
#Spell: DiscoveryWindows
^ Intent: discover models correctly on Windows installs

@WindowsDiscovery
  ~ os = windows
  : username -> Vec<Path>
  ! probes_drive_letters [C,D,E,F]
  - assumes_posix_home
```

If a rehydrator doesn’t support platform-gated invocation, this can be treated as a separate slice gate.

---

### 8) Sharded model grouping (regex-driven grouping)

**Observed:** upstream groups shards like `-00001-of-00004`; rehydration stubbed.

- **Class:** S4 (Regex/format precision)
- **Primary glyphs:** `:` + `!` + `-`

```glyph
#Spell: ShardedModelGrouping
^ Intent: treat sharded weight files as a single discoverable model

@ShardGrouper
  : paths -> Vec<@DiscoveredModel>
  ! groups_by_pattern "^(.+)-\d{5}-of-\d{5}(\..+)$"
  - returns_each_shard_as_model
```

---

## TS examples: export surface variance mapped to glyphs

**Observed:** independent rehydrations often differed on default vs named exports and which helper functions/types were public.

- **Class:** S2 (Underspecified public surface)
- **Primary glyphs:** `@` + `:` + `!` + `-`

**Casting pattern that fixes this without new glyphs:** declare the public surface as an entity and make it exact.

```glyph
#Spell: PublicSurface
^ Intent: define the exact exported API surface

@Exports
  ! export_surface_exact
  - extra_exports

@Export:default
  : value -> clsx

@Export:named
  : clsx(...args) -> string
```

This would have prevented:
- v1 exposing `parse`/`format` (ms) while v2 hides them
- name drift (`equalES6` vs `es6Equal`)

---

## Do we need a new glyph?

Based on the observed gaps, **no missing glyph is required**.

### Why
Every gap can be expressed as some combination of:
- missing entities (`@Subsystem`)
- missing contracts (`: endpoint/signature`)
- missing “exactness” guarantees (`! inventory_exact`, `! export_surface_exact`)
- missing exclusions (`- invented_commands`, `- extra_exports`)
- missing dependencies (`> @Observability`, `> @AnthropicCompat`)
- missing assumptions (`~ os=windows`, `~ protocol=ollama_manifest`)

### What you likely want instead of a new glyph
A **casting rule** (doctrine-level) that says:

> If a set is intended to be exact (commands, routes, exports, supported platforms), you must say so with `!` and forbid invention with `-`, otherwise the invoker is allowed to approximate.

That addresses the core “AI didn’t realize the logic was already baked in” failure mode.

---

## Recommended doctrine/README addendum (wording)

Suggested line to add to the main README:

> Sorcery is not code stenography or a byte-for-byte recorder; it’s a creation and handoff tool that preserves intent, contracts, and constraints—implementation details may differ.

(Next step in repo work: I can apply this as a one-line addition near the existing “design doctrine” note.)
