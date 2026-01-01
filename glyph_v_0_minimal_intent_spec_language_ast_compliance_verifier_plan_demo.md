# Glyph v0: Minimal Intent Spec Language + AST Compliance Verifier

## Purpose
Create a **minimal, push-button** “intent spec” language (Glyph v0) that:
1) Lets a large model/human **dehydrate intent** into a compact spec.
2) Lets a small local model implement from that spec.
3) Lets a tool **verify compliance deterministically** by comparing **Spec-Obligations** to **Code Facts** (AST + a small golden test harness).

Target: a working MVP that can parse Glyph → Spec AST → Obligations, parse code → Fact Graph, and output a **binary verdict** with an actionable diff.

---

## Non-negotiable design constraints
- **Push-button UX:** `glyph init`, `glyph verify` are the core. No training required.
- **Fail-closed:** Missing mapping/evidence/ambiguity = FAIL.
- **Minimal language surface:** Only encode what can be checked.
- **No AI narration dependency:** Facts come from AST + tool outputs, not from LLM descriptions.

---

## What I need to know (exactly) before implementation
If anything is unknown, pick the default and proceed.

1) **Implementation language for the toolchain**
   - Default: **Rust** (fits your ecosystem and local-first).

2) **First code target language to verify**
   - Default: **Rust** (using `syn` for AST) with optional runtime golden tests.

3) **Mapping convention from spec entities to code**
   Choose one (default included):
   - **Default:** `@Name` maps to `struct Name` OR `mod name` OR `fn name_*` (case-insensitive rules). Ambiguous matches FAIL.
   - Optional: allow explicit binding: `@Parser { bind: "crate::parser::Parser" }` (v1).

4) **Exclusion sets (banned APIs) and deterministic list**
   - Default v0 set:
     - `network` bans: `std::net`, `reqwest`, `hyper`, `ureq`, `tokio::net`
     - `filesystem_writes` bans: `std::fs::write`, `File::create`, `OpenOptions::new().write(true)`, `remove_file`, `create_dir`
     - `nondeterminism` bans: `rand`, `SystemTime::now`, `Instant::now`, `thread_rng`, `uuid::Uuid::new_v4`

5) **Golden test harness scope**
   - Default: one executable demo with `cargo test` verifying determinism: same input twice → identical output bytes.

---

## Glyph v0 language spec (minimal)
### Conceptual model
- A file contains one **Spec**.
- A Spec contains:
  - `#` Spec header (name)
  - `^` Intent line
  - 1+ `@` Entities
  - Each Entity has:
    - exactly one `:` contract
    - 0+ `!` guarantees
    - 0+ `-` exclusions
    - 0+ `~` assumptions
  - 0+ `>` dependencies
  - 0+ `?` open questions (ignored by verifier; informational)

### Canonical formatting rules
- Indentation indicates association: lines indented under an `@Entity` belong to it.
- Empty lines allowed.
- Comments: `//` to end of line.
- Text after markers is raw text; verifier interprets only known primitives.

---

## EBNF (Glyph v0)
This is intentionally small.

```
file          = ws, spec, ws;

spec          = header, nl,
                intent, nl,
                { wsline },
                { entity_block | dep_line | question_line | wsline };

header        = "#", name;
intent        = "^", text;

author_line   = "%", text;                // optional future metadata

entity_block  = entity, nl, { indented_line };
entity        = "@", name;

indented_line = indent, ( contract | guarantee | exclusion | assumption | wsline ), nl;

contract      = ":", ws, sig;
guarantee     = "!", ws, text;
exclusion     = "-", ws, text;
assumption    = "~", ws, text;

dep_line      = ">", ws, name, nl;
question_line = "?", ws, text, nl;

sig           = text;                      // v0: raw string, parsed later optionally

name          = namechar, { namechar };
namechar      = letter | digit | "_" | "-" | ".";

text          = { anychar_except_nl };

ws            = { " " | "\t" | "\r" };
indent        = ("  " | "\t"), { "  " | "\t" }; // at least one indent unit
wsline        = ws, [ "//", text ], nl;

nl            = "\n";
```

Notes:
- v0 treats `sig` as an opaque string.
- v1 can formalize `sig` (e.g., `(A,B)->C`), but keep v0 permissive.

---

## ANTLR approach (recommended for speed + tooling)
### Files
- `GlyphLexer.g4`
- `GlyphParser.g4`

### Strategy
ANTLR is line-oriented but can handle indentation with:
- explicit INDENT/DEDENT tokens (Python-style) OR
- simpler: parse entity blocks by consuming lines that start with indent.

For v0, implement a **two-stage parser**:
1) **Line classifier**: scan file into tokens: HEADER, INTENT, ENTITY, INDENTED_LINE, DEP, QUESTION.
2) **ANTLR** parses these tokens into a Spec AST.

This avoids the complexity of true indentation-sensitive grammars.

---

## Spec AST (data structures)
### Core types
- `Spec { name, intent, entities[], deps[], questions[] }`
- `Entity { name, contract: String, guarantees[], exclusions[], assumptions[] }`

### Stable IDs
Assign IDs as: `sha256(normalize("<specpath>#<entityname>#<line-range>"))`
- Used for diff pointers and deterministic output.

---

## Obligation compiler (Spec AST → Obligations)
### Obligation types (v0)
1) `ContractExists`
   - Entity requires at least one code target with matching name mapping.
2) `ContractSignatureRoughMatch` (optional v0)
   - If you define a signature format, compare to function signature.
3) `ExclusionBannedAPI`
   - For each exclusion, apply a banned-symbol scan in AST.
4) `DependencyEdge`
   - If `> Foo`, require imports/uses/calls referencing Foo (best effort).
5) `GuaranteeDeterministic` (runtime harness)
   - If `! deterministic`, require `cargo test -p demo` includes determinism test.

### Interpretation rules
- Unknown `!` and `-` lines are **informational** in v0 unless mapped.
- But if a guarantee/exclusion is present and unsupported by the verifier, default policy is **FAIL** (fail-closed) unless marked `! advisory`.

(You can pick the opposite policy if you prefer: unsupported = WARN. v0 recommends FAIL-closed.)

---

## Fact extraction (Code → Fact Graph)
### Rust extractor (v0)
Use `syn` to parse `.rs` files and extract:
- modules
- structs
- traits
- functions: name, params count, return type string
- paths used (for banned API detection): `syn::ExprPath`, `syn::TypePath`, `use` statements

**FactGraph**
- `defs.structs: set<String>`
- `defs.fns: map<String, FnSig>`
- `uses.paths: multiset<String>`
- `uses.imports: set<String>`

Normalization:
- `std::net::TcpStream` and `TcpStream` both normalize to candidate paths via import resolution (best-effort v0: string match against raw paths + known crate prefixes).

---

## Judge (Obligations × Facts → Verdict)
### Output
- `PASS` or `FAIL`
- Machine JSON: `glyph_verdict.json`
- Human Markdown: `glyph_verdict.md`

### Actionable diff format (minimum)
For each failed obligation:
- Spec pointer: `spec_name`, `entity_name`, `node_id`
- Failure class: `missing_target`, `banned_api`, `dep_missing`, `unsupported_obligation`
- Evidence: file + line (where available)
- Remediation hint: one-liner template

---

## CLI UX (push-button)
### Commands
- `glyph init`
  - writes `.glyph/` defaults:
    - `glyph.toml` (mapping rules, banned API sets)
    - `specs/` directory
    - `examples/` demo spec

- `glyph verify [--spec specs/demo.glyph] [--path .]`
  - parses spec
  - extracts facts
  - judges
  - exits non-zero on FAIL

Optional later:
- `glyph fmt` (format specs)
- `glyph explain` (prints obligations)

---

## Demo program (small but non-trivial)
### Demo: Deterministic Constraint Gate
Implement a small Rust crate `demo_gate` with:
- `Parser` that parses a tiny mini-format (not JSON) into an AST
- `Comparator` that checks whether an “artifact description” includes all required items
- `Verdict` is binary

### Why this demo
It exercises:
- contracts
- exclusions (no filesystem writes, no network)
- determinism (golden tests)
- actionable diff

### Demo structure
- `demo_gate/src/lib.rs`
- `demo_gate/tests/determinism.rs`

Determinism test:
- run `compare(spec, artifact)` twice; assert identical bytes output

---

## Demo Glyph spec (v0)
Create `specs/constraint_gate.glyph`:

```
#ConstraintGate
^ deterministically verify that an artifact description covers a specification’s declared constraints

@Parser
  : constraint_text -> ast
  ! deterministic
  ! stable_normalization
  - network
  - filesystem_writes

@Comparator
  : (spec_ast, artifact_ast) -> verdict
  ! fails_on_missing_intent
  ! fails_on_missing_guarantees
  ! fails_on_missing_exclusions
  ! fails_on_missing_dependencies
  ! emits_actionable_diff
  - guessing
  - nondeterminism

@Verdict
  : input -> pass_fail
  ! binary
  - partial_success

> Parser
> Comparator
```

Notes:
- v0 verifier supports `- network`, `- filesystem_writes`, `- nondeterminism`.
- `! deterministic` is enforced by the demo test harness.
- Unsupported guarantees (`fails_on_missing_*`, `emits_actionable_diff`) can be:
  - marked as advisory in v0, OR
  - implemented as tests for the verifier (v1).

---

## Step-by-step implementation plan (local AI runnable)

### Phase 1 — Repository scaffolding (0.5 day)
1) Create repo: `glyph/`
2) Create Rust workspace:
   - `glyphc` (library): parser + AST + obligations
   - `glyph-cli` (bin): init/verify
   - `glyph-rust-extract` (library): Rust fact extractor
   - `demo_gate` (crate): demo program

Acceptance: `cargo test` runs; `glyph verify` exists but may stub.

---

### Phase 2 — Parser (1–2 days)
Option A (fastest): implement a **line-based parser** in Rust for v0.
- Because v0 is line-oriented, this is simpler than ANTLR initially.
- Still produce the same Spec AST.

Option B (ANTLR): generate parser (TypeScript/Java) and call it.
- Adds build/tool complexity.

**Recommendation for speed:** v0 line parser in Rust; add ANTLR later if needed.

Acceptance:
- Parse `constraint_gate.glyph` into Spec AST.
- Enforce: each `@Entity` must have exactly one `:` contract (else FAIL).

---

### Phase 3 — Obligation compiler (0.5–1 day)
Implement obligation compilation:
- map `- network/fs/nondeterminism` to `BannedAPI` obligations
- map `@Name` to `ContractExists`
- map `> Dep` to `DependencyEdge`

Acceptance:
- `glyph explain` (optional) prints obligations.

---

### Phase 4 — Rust fact extractor (1–2 days)
Implement file walk + `syn` parsing.
Extract:
- struct names
- function names + rough signature string
- used paths / imports

Acceptance:
- Extractor produces FactGraph JSON for `demo_gate`.

---

### Phase 5 — Judge + diff outputs (1 day)
Implement:
- match entity → code targets by naming rules
- banned API detection by scanning extracted `uses.paths`
- dependency check (best effort: references/imports)

Acceptance:
- If `demo_gate` imports `reqwest`, verifier FAILS with pointer.
- If `Parser` missing, FAIL.

---

### Phase 6 — Determinism harness (0.5–1 day)
In `demo_gate/tests/determinism.rs`:
- call core functions twice
- assert identical outputs

In verifier:
- if `! deterministic` present, require test file exists OR require `cargo test` includes a tagged test name (v0 simplest: require `tests/determinism.rs` exists).

Acceptance:
- deleting determinism test causes verifier FAIL.

---

### Phase 7 — Push-button polish (0.5 day)
- `glyph init` writes:
  - `.glyph/glyph.toml` with defaults
  - `specs/constraint_gate.glyph`
  - `demo_gate` crate skeleton
- `glyph verify` runs without configuration

Acceptance:
- New user runs: `cargo install --path glyph-cli` then `glyph init` then `glyph verify` and gets PASS.

---

## v0 Deliverables
- `Glyph v0` parser producing Spec AST
- Rust code fact extractor
- Obligation judge
- Binary verdict + actionable diff
- Demo crate + demo spec

---

## v1+ Extensions (do not block v0)
- Formal contract signature parsing `(A,B)->C` with type normalization
- Better dependency checks via call graph
- Multi-language extractors (TS/Go)
- IDE integration (syntax highlight)
- Optional ANTLR grammar for stronger errors

---

## Risks and mitigations
- **AST cannot prove deep semantics** → use golden tests for `! deterministic` and similar.
- **Macros / dynamic dispatch** → rely on conventions and fail-closed on ambiguity.
- **Banned API false negatives** → expand banned sets; prefer conservative matching.

---

## Success criteria
A single command produces a trustworthy verdict:
- `glyph verify` on the demo repo yields PASS.
- Introduce a forbidden API or remove an entity → FAIL with a concrete pointer.

---

## Local AI prompt (copy/paste)

You are implementing Glyph v0 per this document.
Rules:
- Fail-closed.
- Minimal scope.
- Produce working code with tests.
- Implement phases in order and keep commits small.

Tasks:
1) Scaffold Rust workspace with crates: glyphc, glyph-cli, glyph-rust-extract, demo_gate.
2) Implement v0 line-based parser to Spec AST.
3) Implement obligation compilation.
4) Implement Rust fact extractor using syn.
5) Implement judge + verdict outputs.
6) Implement demo_gate and determinism test.
7) Implement `glyph init` + `glyph verify` UX.

Definition of done:
- `glyph verify` returns exit 0 on PASS, non-zero on FAIL.
- `glyph_verdict.md` explains failures with file pointers.

