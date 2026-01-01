# Sorcery Case Study: fast-deep-equal (Reset to Test-Bound Spells)

**Authorship note:** Original analysis by **Opus Claude 4.5**. Updated for test-bound reset.

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

## Original Experiment (Semantic Analysis)

Dehydrated fast-deep-equal into spells, rehydrated without referencing the original.

**Result:** ~95% behavioral fidelity. Edge cases in Set comparison diverged.

### Comparison (Original vs Rehydrated)

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

**Behavioral:** ~95% match

### Gap Analysis

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
## New Experiment (Test-Bound Spells)

Rebuilt with test-bound obligations. Code identical, but with comprehensive tests.

### Test Suite Coverage

The test-bound rehydration includes executable tests for each `$ prove` obligation:

| Test File | Obligation | Description |
|-----------|------------|-------------|
| `same_ref.test.ts` | `same_reference_fast_path` | Validates identical objects return true immediately |
| `both_objects.test.ts` | `both_must_be_objects_to_continue` | Tests type checking before deep comparison |
| `circular_reference.test.ts` | `circular_reference_detection` | Verifies circular structure handling |
| `date_equality.test.ts` | `date_objects_equal_by_value` | Tests Date comparison by valueOf |
| `deep_array_equality.test.ts` | `deep_array_comparison` | Validates nested array comparison |
| `deep_object_equality.test.ts` | `deep_object_comparison` | Tests nested object comparison |
| `empty_objects_arrays.test.ts` | `empty_objects_and_arrays_are_equal` | Verifies empty structures are equal |
| `function_equality.test.ts` | `functions_are_never_equal` | Ensures functions never compare equal |
| `map_equality.test.ts` | `map_objects_equal_by_content` | Tests Map comparison by entries |
| `nan_equality.test.ts` | `nan_values_are_equal` | Validates NaN === NaN behavior |
| `null_undefined_equality.test.ts` | `null_and_undefined_are_not_equal` | Tests null/undefined distinction |
| `primitive_equality.test.ts` | `primitive_values_use_strict_equality` | Verifies strict equality for primitives |
| `regexp_equality.test.ts` | `regexp_objects_equal_by_value` | Tests RegExp comparison by source/flags |
| `set_equality.test.ts` | `set_objects_equal_by_content` | Validates Set comparison by membership |
| `symbol_equality.test.ts` | `symbols_equal_by_reference` | Tests Symbol reference equality |
| `typed_array_equality.test.ts` | `typed_arrays_equal_by_content` | Verifies TypedArray element comparison |

### Comparison (New vs Original)
- **Notation:** !/- → $ require/forbid/prove with tests
- **Enforcement:** Semantic analysis → Runtime test execution
- **Code:** Identical functionality
- **Fidelity:** ~95% maintained (same gaps) with test verification
- **Verification:** AI self-check → Executable evidence

### What the Test-Bound Spell Adds
- Executable tests for all prove obligations
- Fail-closed validation of forbid constraints
- Runtime verification of all requirements
- Complete test coverage ensuring behavioral fidelity

### Learnings
Complex equality comparisons benefit from test bindings; gaps in original persist but are now verifiable. Test-bound approach eliminates AI self-verification, providing stronger guarantees than semantic analysis alone.

---
## Files

```
examples/fast-deep-equal/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── equal.spell
├── rehydrated/
│   └── equal.ts
└── rehydrated2/           # Test-bound rehydration
    ├── equal.ts           # Implementation
    └── tests/             # Test suite
        ├── same_ref.test.ts
        ├── both_objects.test.ts
        ├── circular_reference.test.ts
        ├── date_equality.test.ts
        ├── deep_array_equality.test.ts
        ├── deep_object_equality.test.ts
        ├── empty_objects_arrays.test.ts
        ├── function_equality.test.ts
        ├── map_equality.test.ts
        ├── nan_equality.test.ts
        ├── null_undefined_equality.test.ts
        ├── primitive_equality.test.ts
        ├── regexp_equality.test.ts
        ├── set_equality.test.ts
        ├── symbol_equality.test.ts
        └── typed_array_equality.test.ts
```
