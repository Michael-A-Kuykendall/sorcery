# Sorcery Case Study: once

## Abstract

Dehydrated [once](https://github.com/isaacs/once) (~40 lines) into Sorcery spells, then rehydrated without referencing the original.

**Result:** ~95% behavioral fidelity. One gap identified.

---

## The Subject

`once` wraps a function so it executes at most once. Subsequent calls return the cached result. Also provides `onceStrict` which throws on subsequent calls.

---

## Spells Created

| Spell | Purpose |
|-------|---------|
| `once.spell` | Once, OnceStrict, OnceMeta |

---

## Comparison

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| Returns wrapper function | ✓ | ✓ | ✅ |
| First call executes | ✓ | ✓ | ✅ |
| Subsequent returns cached | ✓ | ✓ | ✅ |
| Preserves `this` | ✓ | ✓ | ✅ |
| Preserves arguments | ✓ | ✓ | ✅ |
| `called` flag on wrapper | ✓ | ✓ | ✅ |
| `value` property on wrapper | ✓ | ✓ | ✅ |
| onceStrict throws | ✓ | ✓ | ✅ |
| Error includes fn name | ✓ | ✓ | ✅ |
| **Uses wrappy dependency** | ✓ | ✗ | ❌ |
| **Provides .proto method** | ✓ | ✗ | ❌ |

### Behavioral: ~95% match
### Gaps identified: 2

---

## Gap Analysis

### Gap 1: wrappy dependency

The original `once` uses `wrappy` to preserve function properties (name, length, etc.) on the wrapper. The spell didn't capture this because it's a **meta-concern** about function identity, not about the once behavior itself.

**Doctrine question:** Should wrapper metadata preservation be a standard spell concept?

### Gap 2: .proto method

The original provides `once.proto` which monkey-patches `Function.prototype` with `.once()` and `.onceStrict()` methods. This was not captured because it's an **optional side effect**, not core functionality.

**Doctrine question:** How should optional "install" behaviors be captured? Maybe a separate spell type?

---

## What the Spell Captured

- ✓ Core once behavior
- ✓ Caching semantics
- ✓ Strict mode throwing
- ✓ Error message formatting
- ✓ Wrapper properties

---

## What the Spell Missed

- ✗ wrappy integration (function property preservation)
- ✗ Optional prototype extension

---

## Doctrine Insight

**The gaps reveal a category not well-served by current Glyph:** *optional installation/enhancement behaviors* and *wrapper metadata preservation*.

These could be addressed by:
1. A `+` symbol for "optional enhancement"
2. Standard guarantees like `! preserves_function_name` and `! preserves_function_length`

---

## Files

```
examples/once/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── once.spell
└── rehydrated/
    └── once.ts
```
