# Sorcery Reset: Test-Bound Spells (Back-of-Napkin Architecture)

## Purpose

Sorcery is being reset to a **minimal, deterministic system** whose sole job is to:

> **Move architectural intent between large planning models and small local execution models, and clamp correctness using mandatory runnable tests—not AI narration or semantic AST analysis.**

This design explicitly abandons:

* semantic verification via AST comparison
* AI "self-certification"
* CI/CD frameworks
* complex parsers or analyzers

Instead, it enforces **runtime evidence** as the source of truth.

---

## Core Principle

> **A spell is not complete unless its tests run and pass.**

No tests → not done
Tests fail → not done
Missing test coverage → not done

There is no other notion of correctness.

---

## Vocabulary (kept intentionally small)

* **Sorcery**
  The system / methodology.

* **Spell**
  A compact architectural specification that:

  * declares intent
  * declares obligations
  * binds each obligation to an executable test

* **Invoke**
  The act of generating code (human or AI) from a spell.

* **Verify**
  The act of running all tests bound by a spell and enforcing fail-closed rules.

These words are **labels**, not metaphors. They exist only to keep the mental model simple.

---

## What a Spell Is (and Is Not)

### A Spell IS:

* A **dehydrated architectural plan**
* A **checklist of obligations**
* A **binding contract to tests**
* A **unit of work that can be verified with one command**

### A Spell IS NOT:

* A proof of correctness
* A semantic description of algorithms
* A replacement for tests
* A thing the AI gets to judge itself

---

## Spell Structure (Minimal)

A spell contains four kinds of information:

### 1. Intent (non-gating, human-readable)

```text
^ Intent: Deterministically verify artifact constraints
```

This exists for planning and comprehension only.

---

### 2. Obligations (gating)

Obligations are **machine-enforced** and must be one of:

* `require` → something must exist
* `forbid` → something must not exist
* `prove`  → a property must be demonstrated by a test

Example:

```text
$ require: fn parse
$ forbid: network
$ prove: deterministic -> test: determinism
```

Rules:

* Every `$` line is **mandatory**
* Unknown `$` types → FAIL
* Unbound `prove` → FAIL

---

### 3. Test Bindings (mandatory for `prove`)

Every `prove` obligation must bind to a test:

```text
$ prove: deterministic -> test: determinism
```

The spell does **not** define how the test works—only that:

* it exists
* it runs
* it passes

---

### 4. Implementation (external)

The spell does **not** contain code.
The spell requires code + tests to exist somewhere in the repo.

---

## Verification Rules (Fail-Closed)

Verification is intentionally dumb and absolute.

### Verification does exactly this:

1. Parse the spell.
2. Collect all `$` obligations.
3. For each obligation:

   * `require` → check symbol existence (name-based, shallow)
   * `forbid`  → scan for forbidden capability (string or token scan)
   * `prove`   → confirm bound test exists and ran
4. Run all bound tests.
5. If **anything** is missing, unsupported, or fails → **FAIL**

There is no partial success.
There is no "close enough."
There is no AI explanation step.

---

## Tests Are the Semantic Escape Hatch

Semantic intent **is not inferred from code**.

Semantic intent is proven by **tests**.

Examples:

* Determinism → golden-input test
* Stable normalization → golden-output test
* Error handling → explicit failure-mode tests
* Boundary enforcement → negative tests

This is not a compromise—it is the *correct* way to close the loop.

---

## Why This Works (and Scales)

* Large models are good at **planning** → they write spells.
* Small models are good at **execution** → they implement code + tests.
* Sorcery is good at **enforcement** → it runs tests and clamps drift.

This prevents:

* embellishment
* scope creep
* silent violations
* "AI shook its own hand"

---

## What Was Explicitly Removed

The following are **intentionally not part of Sorcery** anymore:

* AST semantic comparison
* Code → glyph → glyph comparison
* CI/CD frameworks
* Language-specific analyzers
* NLP intent reconstruction
* "understanding" code

If something cannot be tested, it cannot be enforced.

---

## CLI Surface (Deliberately Small)

Only two commands matter:

```bash
invoke <spell>
verify <spell>
```

* `invoke` = generate or guide implementation (human or AI)
* `verify` = run tests + enforce obligations

Everything else is optional tooling.

---

## Definition of "Done"

A spell is **DONE** if and only if:

* All `$ require` obligations are satisfied
* All `$ forbid` obligations are satisfied
* All `$ prove` obligations have bound tests
* All bound tests ran and passed

Anything else is incomplete.

---

## Design Ethos (Non-Negotiable)

* Back-of-the-napkin friendly
* Deterministic
* Fail-closed
* Test-first
* No semantic ambition
* No frameworks
* No AI trust

---

## Why This Is the Right Reset

This preserves:

* the **spells/sorcery mental model**
* the **intent dehydration** idea
* the **large-to-small AI workflow**

While eliminating:

* semantic-analysis dead ends
* verification theater
* framework explosion
* brittle AST gymnastics

---

## Final Statement

> **Sorcery is no longer about proving intent.
> Sorcery is about refusing to accept work without evidence.**

That is the system.

---

## Integration with Glyph v0
- Retain core symbols (#, ^, @, :, !, -, ~, >, ?).
- Add $ for obligations: $ require/forbid/prove.
- Update parser/verifier to handle $ lines and test bindings.
- Verifier: Check coverage, run tests (e.g., via cargo test with filters).
- Demo: Update demo.glyph with $ obligations, add test bindings.</content>
<parameter name="filePath">c:\Users\micha\repos\sorcery\Sorcery_Reset_Test_Bound_Spells.md