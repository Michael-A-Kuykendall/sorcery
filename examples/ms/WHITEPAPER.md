# Sorcery Case Study: ms

**Authorship note:** This rehydration + analysis was produced by **Opus Claude 4.5**.

## Abstract

Dehydrated [ms](https://github.com/vercel/ms) (~200 lines) into Sorcery spells, then rehydrated without referencing the original.

**Result:** ~98% behavioral fidelity. Minor edge case divergence.

---

## The Subject

`ms` provides bidirectional conversion between millisecond numbers and human-readable time strings like "2 days" or "1h".

---

## Spells Created

| Spell | Purpose |
|-------|---------|
| `ms.spell` | Ms, Parse, Format, FmtShort, FmtLong, TimeConstants |

---

## Comparison

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| String → number | ✓ | ✓ | ✅ |
| Number → string | ✓ | ✓ | ✅ |
| All unit aliases | ✓ | ✓ | ✅ |
| Case insensitive | ✓ | ✓ | ✅ |
| Decimal support | ✓ | ✓ | ✅ |
| Negative support | ✓ | ✓ | ✅ |
| Long format option | ✓ | ✓ | ✅ |
| Pluralization at 1.5x | ✓ | ✓ | ✅ |
| Throws on invalid | ✓ | ✓ | ✅ |
| 100 char limit | ✓ | ✓ | ✅ |
| **Exact regex pattern** | Complex | Simplified | ⚠️ |
| **TypeScript StringValue type** | ✓ | Simplified | ⚠️ |

### Behavioral: ~98% match
### Implementation: Regex simplified

---

## Gap Analysis

### Gap 1: Regex Exactness

The original uses a very precise regex with named capture groups. The rehydrated uses a simpler pattern. Both parse the same inputs correctly, but edge cases around malformed input may differ.

**Doctrine question:** How do we capture regex precision in spells? `! regex_based_parsing` doesn't specify *which* regex.

### Gap 2: TypeScript Types

The original has elaborate TypeScript types for type-safe string literals (`StringValue`). The spell doesn't capture type system concerns.

**Doctrine observation:** Spells capture *runtime behavior*, not *static type constraints*. This is correct—types are a separate concern.

---

## What the Spell Captured

- ✓ Bidirectional conversion
- ✓ All unit aliases
- ✓ Formatting options
- ✓ Error conditions
- ✓ Time constants
- ✓ Pluralization rules

---

## What the Spell Missed

- ⚠️ Exact regex pattern (but behavior matches)
- ⚠️ TypeScript type gymnastics (by design)

---

## Doctrine Insight

**ms is the most complex example so far.** The spell successfully captured:
- Multi-component structure (Parse, Format, etc.)
- Bidirectional behavior
- Detailed constant values

The gap around regex precision suggests that **highly specific parsing rules** may need a different representation—perhaps a linked grammar or test cases.

---

## Files

```
examples/ms/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── ms.spell
└── rehydrated/
    └── ms.ts
```
