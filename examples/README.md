# Sorcery Examples

This directory contains real-world case studies demonstrating Sorcery's dehydration/rehydration cycle.

## What's Happening Here

Each subdirectory takes a **real, public, well-known library** and:

1. **Dehydrates** it — reads the source, extracts intent, encodes as spells
2. **Rehydrates** it — uses only the spells to rebuild the code from scratch
3. **Compares** — measures what survived the round-trip

## Structure

Every example follows the same layout:

```
example-name/
├── README.md           # What this library does
├── WHITEPAPER.md       # Full analysis of the experiment
├── spells/             # The dehydrated intent
│   ├── component.spell
│   └── ...
└── rehydrated/         # Code rebuilt from spells alone
    └── implementation.ts
```

## The Experiment

**Question:** Can spells capture enough intent that an agent could rebuild a library without seeing the original code—and produce something functionally equivalent?

**Method:** 
- Read original source once
- Extract intent into spells (guarantees, exclusions, assumptions, contracts)
- Close the original source
- Generate new implementation from spells only
- Compare behavioral fidelity

## Current Examples

| Directory | Type | Description | Fidelity |
|-----------|------|-------------|----------|
| [syntax/](syntax/) | Reference | Standalone spells demonstrating Glyph notation | — |
| [mitt/](mitt/) | Case Study | Event emitter | 100% |
| [dlv/](dlv/) | Case Study | Deep object access | 100% |
| [clsx/](clsx/) | Case Study | Classname builder | 100% |
| [once/](once/) | Case Study | Call-once wrapper | ~95% |
| [ms/](ms/) | Case Study | Time string parser | ~98% |
| [fast-deep-equal/](fast-deep-equal/) | Case Study | Deep equality | ~95% |
| [shimmy/](shimmy/) | Case Study | Rust inference server (4.8MB) | ~85% |

## Cross-Model Validation

This experiment was run twice with different AI models:
- **v1 (`rehydrated/`)**: Claude Opus 4.5 — original spell author and first rehydrator
- **v2 (`rehydrated2/`)**: GPT-5.2 — independent rehydration from spells only

See **[WHITEPAPER_EXPERIMENT_ASSESSMENT.md](WHITEPAPER_EXPERIMENT_ASSESSMENT.md)** for the full cross-model analysis, including:
- Why failures were *casting gaps*, not *invocation errors*
- How improper glyph usage (not using `>` dependencies) caused inventory drift
- Evidence that higher-context AI would prevent observed divergences
- Confirmation that the 9-symbol grammar is complete

## Insights from Case Studies

### What Spells Capture Well
- Behavioral contracts (input → output)
- Guarantees and invariants
- Explicit exclusions
- Composition and dependencies
- Recursive behavior

### Where Gaps Emerged
- **Wrapper metadata** (function name/length preservation)
- **Optional enhancements** (prototype extensions)
- **Regex precision** (exact patterns vs. described behavior)
- **Library variants** (React-specific, ES6 versions)
- **Implicit language semantics** (Set.has uses ===)

### Doctrine Refinements Suggested
1. Standard guarantees for wrapper preservation
2. A pattern for library variants
3. Acknowledgment that spells assume language knowledge

### New: Context Asymmetry (Casting vs Invocation)
These experiments also showed that casting is the high-context task and invocation is the low-context task.
See [LESSONS_CONTEXT_ASYMMETRY.md](LESSONS_CONTEXT_ASYMMETRY.md).

## Meta-Analysis Documents

| Document | Purpose |
|----------|---------|
| [WHITEPAPER_EXPERIMENT_ASSESSMENT.md](WHITEPAPER_EXPERIMENT_ASSESSMENT.md) | **Full experimental validation** — cross-model analysis by Opus 4.5 |
| [INDEPENDENT_EVALUATION_GPT5.2.md](INDEPENDENT_EVALUATION_GPT5.2.md) | **Third-party evaluation** — GPT-5.2's assessment of the experiment |
| [LESSONS_CONTEXT_ASYMMETRY.md](LESSONS_CONTEXT_ASYMMETRY.md) | Key insight: casting is high-context, invocation is low-context |
| [GLYPH_GAP_TO_GLYPH_MAPPING.md](GLYPH_GAP_TO_GLYPH_MAPPING.md) | Proof that existing glyphs suffice for all observed gaps |
| [REHYDRATION_COMPARISON_V1_V2.md](REHYDRATION_COMPARISON_V1_V2.md) | Detailed v1 vs v2 diff analysis |

## Adding Examples

When adding a new example:

1. Choose a small, public, well-known library
2. Create the directory structure above
3. Write spells that capture intent, not implementation
4. Rehydrate without looking at original
5. Document what survived and what diverged

The goal is **not** to produce identical code. The goal is to produce **functionally equivalent** code that honors all contracts and exclusions.

---

*Sorcery — because intent deserves to survive transmission.*
