# Glyph Master Plan: Minimal Intent Spec Language + AST Compliance Verifier

## Executive Summary
This master plan merges the high-level evolution from Sorcery_Spec_Evolution.md (context, comparisons, AI integration) with the detailed Glyph v0 plan (concrete spec, EBNF, verifier). Goal: Build Glyph as an AI-teachable language for dehydrating intent into enforceable specs, with deterministic rehydration/validation. Push-button, fail-closed, minimal—get it done.

Unique additions from Sorcery_Spec_Evolution.md:
- AI-teaching framework (prompts, training data).
- Compliance contract draft.
- Broader comparisons (e.g., to BDD, formal methods).
- Vision for codegen alongside validation.

## Purpose (from Glyph v0 Plan)
Create a **minimal, push-button** "intent spec" language (Glyph v0) that:
1. Lets a large model/human **dehydrate intent** into a compact spec.
2. Lets a small local model implement from that spec.
3. Lets a tool **verify compliance deterministically** by comparing **Spec-Obligations** to **Code Facts** (AST + a small golden test harness).

Target: Working MVP with parsing, verification, binary verdict + actionable diff.

## Non-Negotiable Design Constraints (from Glyph v0 Plan)
- **Push-Button UX:** `glyph init`, `glyph verify` core commands. No training.
- **Fail-Closed:** Missing mapping/evidence/ambiguity = FAIL.
- **Minimal Language Surface:** Encode only checkable elements.
- **No AI Narration Dependency:** Facts from AST/tools, not LLM descriptions.

## Glyph v0 Language Spec (from Glyph v0 Plan, Aligned with Sorcery Notation)
### Conceptual Model
- File: One **Spec**.
- Spec: `#` header, `^` intent, 1+ `@` entities, 0+ `>` deps, 0+ `?` questions.
- Entity: `@Name`, one `:` contract, 0+ `!` guarantees, 0+ `-` exclusions, 0+ `~` assumptions.

### Canonical Formatting
- Indentation associates lines to entities.
- Empty lines/comments (`//`) allowed.
- Text after markers: Raw; verifier interprets primitives.

## EBNF (from Glyph v0 Plan)
```
file          = ws, spec, ws;
spec          = header, nl, intent, nl, { wsline }, { entity_block | dep_line | question_line | wsline };
header        = "#", name;
intent        = "^", text;
entity_block  = entity, nl, { indented_line };
entity        = "@", name;
indented_line = indent, ( contract | guarantee | exclusion | assumption | wsline ), nl;
contract      = ":", ws, sig;
guarantee     = "!", ws, text;
exclusion     = "-", ws, text;
assumption    = "~", ws, text;
dep_line      = ">", ws, name, nl;
question_line = "?", ws, text, nl;
sig           = text;  // v0: raw string
name          = namechar, { namechar };
```

## Implementation Details (from Glyph v0 Plan + Sorcery Evolution)
- **Toolchain Language:** Rust (fits ecosystem; use `syn` for AST).
- **Target Code Language:** Rust (extend later).
- **Mapping Convention:** `@Name` → `struct Name` OR `mod name` OR `fn name_*` (case-insensitive; ambiguous = FAIL).
- **Exclusion Sets:** Network (`std::net`, etc.), filesystem writes (`std::fs::write`, etc.), nondeterminism (`rand`, etc.).
- **Golden Tests:** `cargo test` for determinism (same input → identical output).

## Verifier Design (from Glyph v0 Plan)
- Parse Glyph → Spec AST → Obligations (guarantees/exclusions).
- Parse Code → Fact Graph (AST + tests).
- Compare: Binary verdict + diff. Fail on missing evidence.

## AI-Teaching Framework (Unique from Sorcery Evolution)
- **Prompts:** "Convert discussion to Glyph spec; ensure ! guarantees explicit."
- **Training Data:** Examples with explanations (e.g., "! means must-have").
- **Compliance:** AI outputs validated; non-match = invalid.
- **Workflow:** Dehydrate ideas in talks → Glyph → Rehydrate (validate/codegen) → Enforce in CI.

## Compliance Contract (Unique from Sorcery Evolution)
1. **Failure Causes:** Missing intent/guarantees/exclusions/deps; ambiguous mappings; violated exclusions.
2. **Satisfaction Artifacts:** Code entities matching @; AST evidence for !/-; passing golden tests.
3. **Absence Handling:** Silence/ambiguity = FAIL (anti-hallucination).

## Comparisons (from Sorcery Evolution)
- **Vs. OpenAPI/BDD:** Glyph adds behavioral obligations; not just schemas/stories.
- **Vs. Formal Methods:** Pragmatic (audit vs. proof); compressive.
- **Novelty:** Absence-as-failure; AI-hostile enforcement.

## Implementation Steps (Merged)
1. **Draft Full EBNF/ANTLR Grammar:** Based on above; add to project.
2. **Build Parser:** Rust/ANTLR → AST.
3. **Implement Verifier:** Compare spec obligations to code facts.
4. **Add Codegen (Optional):** Generate Rust from Glyph (contracts → fns).
5. **Integrate AI:** Test prompts; validate outputs.
6. **Prototype:** `glyph init` (scaffold spec), `glyph verify` (check code).
7. **CI Enforcement:** Fail PRs on non-compliance.

## Risks & Mitigations
- **Grammar Bloat:** Keep minimal; iterate.
- **Tooling Cost:** Start Rust-only.
- **AI Dependency:** Verifier is deterministic; AI for spec creation only.

This is the actionable master plan—execute it.</content>
<parameter name="filePath">c:\Users\micha\repos\sorcery\Glyph_Master_Plan.md