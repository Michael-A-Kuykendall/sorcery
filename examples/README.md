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

| Directory | Type | Description |
|-----------|------|-------------|
| [syntax/](syntax/) | Reference | Standalone spells demonstrating Glyph notation |
| [mitt/](mitt/) | Case Study | Event emitter — 100% behavioral match |

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
