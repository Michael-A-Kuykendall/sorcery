# Sorcery

*A slice‑gated architecture doctrine for compressing high‑context intent into invocable design artifacts.*

---

## 0. One‑Sentence Definition (Napkin Version)

**Sorcery is the practice of casting high‑context intent into constrained spells so lower‑context agents can act correctly without shared memory.**

If a rule or feature cannot be explained in service of that sentence, it does not belong.

---

## 1. Why Sorcery Exists

Modern software development fails at *handoff*.

High‑context reasoning — architectural vision, deep constraints, non‑obvious tradeoffs, and original motivation — is routinely lost when work is delegated to:

- junior engineers
- AI agents with smaller or shifting context windows
- future versions of yourself

Sorcery exists to make intent **portable**, **durable**, and **unambiguous**.

It replaces narrative documentation with **invocable design artifacts**.

---

## 2. Core Principle

> **High‑context intelligence casts intent.  
> Low‑context intelligence invokes intent.**

Sorcery enforces **asymmetry by design**:

- thinking happens once, at the top
- execution happens many times, downstream
- the artifact in between must be complete and context‑poor

Sorcery is explicitly about **transmission**, not execution.

---

## 3. Fundamental Vocabulary

### 3.1 Sorcery (The Doctrine)

**Sorcery** is not magic.

It is disciplined compression.

Sorcery encodes *what must be true* and *what must never happen* so execution does not require guesswork, shared memory, or narrative context.

---

### 3.2 Spells (Slice‑Gated Units)

A **Spell** is the atomic unit of Sorcery.

A spell:

- does exactly one thing
- declares its **Intent** (why it exists)
- declares its **Guarantees** (what must always hold)
- declares its **Assumptions** (what it relies on)
- declares its **Exclusions** (what it forbids)

A spell is either **complete** or **malformed**.

---

### 3.3 Casting and Invocation

- **Casting** is collapsing high‑context reasoning into a spell.
- **Invocation** is expanding a spell into implementation, tests, or documentation.

Casting destroys excess context.  
Invocation never invents missing intent.

Sorcery **ends at invocation**.

---

## 4. Required Structure of a Spell

Every spell MUST contain all of the following:

1. A name
2. A required **Intent**
3. At least one declared entity
4. Explicit guarantees and/or exclusions

A spell without intent is invalid.

---

## 5. Intent (Mandatory)

Every spell must declare a single, bounded **Intent**.

**Intent answers exactly one question:**

> *What outcome must remain true even if tradeoffs are required?*

Intent is:

- the north star
- the tie‑breaker
- the reason the spell exists

Intent is **not**:

- narrative history
- business justification
- implementation guidance
- a list of aspirations

Intent must be concise (1–2 lines).  
A vague intent is equivalent to no intent.

---

## 6. Glyph — The Shorthand Language

**Glyph** is the symbolic notation used to write spells.

Glyph is:

- terse
- structural
- hostile to prose
- optimized for AI or tool invocation

Glyph encodes **intent and constraint**, not explanation.

Markdown, code, and tests are downstream.

---

## 7. Glyph Symbols (v1)

| Symbol | Meaning |
|------|--------|
| `#` | Spell identifier |
| `^` | Intent (required) |
| `@` | Entity / Component |
| `:` | Contract (input → output) |
| `!` | Guarantee / Invariant |
| `~` | Assumption |
| `-` | Explicit exclusion |
| `>` | Dependency (requires) |
| `?` | Open question (blocks gating) |

### 7.1 Dependency Patterns (Using `>`)

`>` has one meaning: **requires**.

Use `:` for flow/shape (`input -> output`). Use `>` only for architectural dependency.

To disambiguate intent without new glyphs, use **naming** plus **delta lines** (`!`/`-`/`~`).

**Pattern A — Prerequisite dependency** (cannot be correct without)

```
@Parser
  > @Tokenizer
```

**Pattern B — Optional extension layering** (extension depends on core)

```
@Core:Tokenizer
  : utf8 -> tokens
  ! deterministic

@Extension:Tokenizer_Tracing
  > @Core:Tokenizer
  ~ optional
  ! core_contract_unchanged
```

**Pattern C — Variant refinement** (variant depends on base, with explicit delta)

```
@Base:Server
  : request -> response
  ! openai_compat

@Variant:Server_AnthropicCompat
  > @Base:Server
  ! adds_anthropic_compat
  - supports_sse
  ~ requires_api_key
```

**Lint (quick check)**

- Every `>` targets a **named** component (`@Something`), not prose.
- Every `@Extension`/`@Variant` includes at least one delta line (`!`/`-`/`~`) explaining what changes.
- If a set must be exact, enumerate dependencies with `>` (unlisted = does not exist).

---

## 8. Example Spell

```
#Spell: Tokenize
^ Intent: produce stable, cross‑platform tokens to enable deterministic inference and caching

@Tokenizer
  : utf8 -> tokens
  ! deterministic
  ! locale_free
  - network
  - persistence
  ~ valid_utf8
```

This spell declares:

- what it does
- why it exists
- what must always be true
- what it assumes
- what it forbids

It intentionally omits implementation details.

---

## 9. Slice Gating Rules

A spell may be gated and invoked only when:

1. An Intent is present
2. No open questions (`?`) remain
3. All guarantees (`!`) are explicit
4. At least one exclusion (`-`) is declared
5. The spell does exactly one thing

A malformed spell cannot be invoked.

### De-Sorcery Cleanup Step

Before shipping an invocation, strip Sorcery jargon from the artifact:

- Add an exclusion such as `- sorcery_terms_in_artifacts` to any production spell.
- Rename components, payload text, and user-facing strings so they no longer contain Sorcery vocabulary (`spell`, `glyph`, `sigil`, `sorcery`, `invocation`, `incantation`, etc.).
- Invocation is not complete until the sigil validates the cleaned artifact (forbidden terms will cause `NOT BOUND`).

---

## 10. Composition

Spells compose only through explicit contracts.

```
#Spell: Parse
^ Intent: build a stable syntax tree for downstream semantic analysis

@Parser
  : tokens -> ast
  > @Tokenizer
  ! no_side_effects
  - io
```

There is:

- no hidden coupling
- no shared narrative
- no implied behavior

Composition is mechanical and auditable.

---

## 11. Negative Space as Design

In Sorcery, **what is forbidden matters as much as what is allowed**.

Exclusions are first‑class.

If a spell declares no exclusions, it is almost certainly incomplete.

---

## 12. What Sorcery Is Not

Sorcery is not:

- a programming language
- a testing framework
- a formal verification system
- a runtime orchestrator
- a notebook workflow

Sorcery defines **design transmission**, not execution mechanics.

---

## 13. Boundary with Testing and Verification

Sorcery defines **what must be true**.

Testing, invariants, contract tests, property tests, SMT, and oracles ensure **it stays true**.

They live **downstream** of Sorcery.

Spells may be consumed by tests.  
Spells must never depend on tests.

---

## 14. Failure Mode to Avoid

**Context Smuggling**:

- relying on shared memory
- relying on narrative documents
- relying on "as discussed earlier"

If intent is not encoded, it is lost.

---

## 15. Canonical Phrase

> **Write spells that say exactly what must happen and what must never happen — so weaker agents can act without guessing.**

---

## 16. Design Ethos

- Compression over verbosity
- Constraints over explanation
- Interfaces over narrative
- Explicit exclusion over implied scope
- Authority in artifacts, not memory

---

## 17. Final Rule

If you cannot remember Sorcery on the back of a napkin, you have added too much.

