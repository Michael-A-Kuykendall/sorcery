# Sorcery Case Study: clsx

## Abstract

Dehydrated [clsx](https://github.com/lukeed/clsx) (~30 lines) into Sorcery spells, then rehydrated without referencing the original.

**Result:** 100% behavioral fidelity.

---

## The Subject

`clsx` builds className strings from mixed inputs: strings, numbers, objects (truthy keys), arrays (recursive), filtering all falsy values.

---

## Spells Created

| Spell | Purpose |
|-------|---------|
| `clsx.spell` | Main function + ToVal helper |

---

## Comparison

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| Variadic arguments | ✓ | ✓ | ✅ |
| Strings pass through | ✓ | ✓ | ✅ |
| Numbers pass through | ✓ | ✓ | ✅ |
| Objects: truthy keys | ✓ | ✓ | ✅ |
| Arrays: recursive | ✓ | ✓ | ✅ |
| Filters falsy | ✓ | ✓ | ✅ |
| Space-separated output | ✓ | ✓ | ✅ |
| Uses `str && (str += ' ')` | ✓ | Uses `if (str) str += ' '` | ⚠️ |
| Uses `var` | ✓ | Uses `let` | ⚠️ |

### Behavioral: 100% match
### Implementation: Minor idiom differences

The original uses short-circuit assignment `str && (str += ' ')`. The rehydrated uses explicit `if`. Identical behavior.

---

## What the Spell Captured

- ✓ All input types handled
- ✓ Recursive array handling
- ✓ Object key filtering by truthiness
- ✓ Space-joined output
- ✓ Falsy filtering

---

## What the Spell Missed

Nothing. The ToVal internal helper was captured as a sub-component.

---

## Doctrine Observation

**Recursive structures (arrays of arrays) are naturally captured by `handles_array_recursively`.** The spell doesn't need to specify recursion depth—"recursively" implies unbounded.

---

## Files

```
examples/clsx/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── clsx.spell
└── rehydrated/
    └── clsx.ts
```
