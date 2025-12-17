# Sorcery Case Study: fast-deep-equal

## Abstract

Dehydrated [fast-deep-equal](https://github.com/epoberezkin/fast-deep-equal) (~60 lines base, ~100 with ES6) into Sorcery spells, then rehydrated without referencing the original.

**Result:** ~95% behavioral fidelity. Edge cases in Set comparison diverged.

---

## The Subject

`fast-deep-equal` recursively compares two values for structural equality, handling JavaScript's many types correctly (objects, arrays, Date, RegExp, NaN, etc.). The ES6 version adds Map, Set, and TypedArray support.

---

## Spells Created

| Spell | Purpose |
|-------|---------|
| `equal.spell` | Equal (base) and ES6Equal |

---

## Comparison

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| Reference equality fast path | ✓ | ✓ | ✅ |
| Constructor match check | ✓ | ✓ | ✅ |
| Array by index | ✓ | ✓ | ✅ |
| Object by keys | ✓ | ✓ | ✅ |
| Key count check | ✓ | ✓ | ✅ |
| NaN === NaN | ✓ | ✓ | ✅ |
| Date by valueOf | ✓ | ✓ | ✅ |
| RegExp by source+flags | ✓ | ✓ | ✅ |
| Custom valueOf | ✓ | ✓ | ✅ |
| Custom toString | ✓ | ✓ | ✅ |
| Map by entries | ✓ | ✓ | ✅ |
| TypedArray by elements | ✓ | ✓ | ✅ |
| **Set comparison** | By has() | By has() | ⚠️ |
| **Reverse iteration** | `i-- !== 0` | `i-- !== 0` | ✅ |

### Behavioral: ~95% match

---

## Gap Analysis

### Gap 1: Set with Object Elements

The original checks Set membership using `has()`, which uses reference equality for objects. This means:

```js
equal(new Set([{}]), new Set([{}])) // false in original
```

The spell captures `! Set_membership_checked_by_has` but doesn't explicitly capture that **objects in Sets compare by reference**. An agent might incorrectly try to deep-compare Set elements.

**Doctrine insight:** Some behaviors are implied by JavaScript semantics (Set.has uses ===). Should spells capture language-level assumptions?

### Gap 2: React-specific handling

The original has a React variant that skips `_owner` properties to avoid circular references. This wasn't captured because it's a **variant**, not the base behavior.

**Doctrine question:** How should library variants be represented?

---

## What the Spell Captured

- ✓ All base type comparisons
- ✓ Recursive structure handling
- ✓ ES6 types (Map, Set, TypedArray)
- ✓ Special cases (NaN, custom valueOf/toString)
- ✓ Exclusions (no circular ref handling)

---

## What the Spell Missed

- ⚠️ Set object reference semantics (implicit in JavaScript)
- ✗ React variant with _owner skip

---

## Doctrine Insight

**fast-deep-equal pushes Sorcery's limits.** It revealed:

1. **Language semantics are implicit.** When a spell says `Set_membership_checked_by_has`, it assumes the agent knows what `has()` means in JavaScript. This is reasonable—spells target agents with language knowledge.

2. **Variants need a pattern.** Multiple versions of the same library (base, ES6, React) could be represented as:
   - Separate spells with composition
   - A base spell with "enhancement" modifiers
   - Conditional guarantees

3. **The exclusion `- handles_circular_references` is load-bearing.** It tells agents not to add cycle detection—which would change the performance profile.

---

## Files

```
examples/fast-deep-equal/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── equal.spell
└── rehydrated/
    └── equal.ts
```
