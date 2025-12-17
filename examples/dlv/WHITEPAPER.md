# Sorcery Case Study: dlv

## Abstract

Dehydrated [dlv](https://github.com/developit/dlv) (~20 lines) into Sorcery spells, then rehydrated without referencing the original.

**Result:** 100% behavioral fidelity.

---

## The Subject

`dlv` safely accesses nested object properties via a dot-path string or array. Returns a default value if any segment is nullish.

---

## Spells Created

| Spell | Purpose |
|-------|---------|
| `dlv.spell` | The single function |

This library is simple enough to be a single spell.

---

## Comparison

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| Accepts string path | ✓ | ✓ | ✅ |
| Accepts array path | ✓ | ✓ | ✅ |
| Splits on dots | ✓ | ✓ | ✅ |
| Returns default if missing | ✓ | ✓ | ✅ |
| Short-circuits on null/undefined | ✓ | ✓ | ✅ |
| Uses `key.split ?` check | ✓ | Uses typeof | ⚠️ |
| Uses for loop with p param | ✓ | Standard for loop | ⚠️ |

### Behavioral: 100% match
### Implementation: Minor divergence

The original uses a clever `key.split ?` check (duck-typing for string). The rehydrated uses `typeof`. Same behavior, different idiom.

---

## What the Spell Captured

- ✓ Input contract (obj, path, default)
- ✓ Output contract (value or default)
- ✓ Path handling (string/array)
- ✓ Nullish short-circuit behavior
- ✓ All exclusions (no throwing, no cloning)

---

## What the Spell Missed

Nothing behavioral. The spell fully captured the intent.

---

## Doctrine Observation

**dlv is small enough that the spell is nearly 1:1 with the code.** This is expected—Sorcery shines more on larger components where compression matters.

---

## Files

```
examples/dlv/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── dlv.spell
└── rehydrated/
    └── dlv.ts
```
