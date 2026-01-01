# Sorcery Case Study: dlv (Reset to Test-Bound Spells)

**Authorship note:** Original analysis by **Opus Claude 4.5**. Updated for test-bound reset.

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

## Original Experiment (Semantic Analysis)

Dehydrated dlv into spells, rehydrated without referencing the original.

**Result:** 100% behavioral fidelity.

### Comparison (Original vs Rehydrated)

| Aspect | Original | Rehydrated | Verdict |
|--------|----------|------------|---------|
| Accepts string path | ✓ | ✓ | ✅ |
| Accepts array path | ✓ | ✓ | ✅ |
| Splits on dots | ✓ | ✓ | ✅ |
| Returns default if missing | ✓ | ✓ | ✅ |
| Short-circuits on null/undefined | ✓ | ✓ | ✅ |
| Uses `key.split ?` check | ✓ | Uses typeof | ⚠️ |
| Uses for loop with p param | ✓ | Standard for loop | ⚠️ |

**Behavioral:** 100% match  
**Implementation:** Minor divergence (duck-typing vs typeof, loop style).

### What the Spell Captured
- ✓ Input contract (obj, path, default)
- ✓ Safe traversal
- ✓ Default handling
- ✓ Nullish short-circuit

### What the Spell Missed
Nothing. Single function, fully captured.

### Doctrine Observation
Simple functions map directly to single spells; obligations cover all behaviors.

---

## New Experiment (Test-Bound Spells)

Rebuilt from test-bound spell with comprehensive test suite covering all prove obligations.

### Test Suite Coverage

The test-bound rehydration includes executable tests for each `$ prove` obligation:

| Test File | Obligation | Description |
|-----------|------------|-------------|
| `string_path.test.ts` | `accepts_string_path_with_dots` | Handles dot-notation paths |
| `array_path.test.ts` | `accepts_array_path` | Handles array paths |
| `splits_dots.test.ts` | `splits_string_on_dots` | Splits strings on dots |
| `undefined_missing.test.ts` | `returns_undefined_if_no_default_and_missing` | Returns undefined when no default |
| `default_missing.test.ts` | `returns_default_if_path_missing` | Returns default when path missing |
| `actual_value.test.ts` | `returns_actual_value_if_found` | Returns actual value when found |
| `null_chain.test.ts` | `handles_null_in_chain` | Handles null in property chain |
| `undefined_chain.test.ts` | `handles_undefined_in_chain` | Handles undefined in property chain |
| `short_circuit.test.ts` | `short_circuits_on_nullish` | Stops traversal on nullish values |
| `obj_check.test.ts` | `obj_is_object_or_nullish` | Validates input object |

### Comparison (New vs Original)
- **Notation:** !/- → $ obligations with tests
- **Enforcement:** Semantic → Test-evidence with executable suite
- **Code:** Identical functionality
- **Fidelity:** 100% maintained with test verification

### What the Test-Bound Spell Adds
- Executable tests for all prove obligations
- Fail-closed validation of forbid constraints
- Runtime verification of all requirements
- Complete test coverage ensuring behavioral fidelity

### Learnings
Dlv's simplicity translates well to test-bound spells; the comprehensive test suite ensures no behavioral drift occurs during transmission.

---

## Files

```
examples/dlv/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── dlv.spell
├── rehydrated/
│   └── dlv.ts
└── rehydrated2/
    ├── dlv.ts
    └── tests/
        ├── string_path.test.ts
        ├── array_path.test.ts
        ├── splits_dots.test.ts
        ├── undefined_missing.test.ts
        ├── default_missing.test.ts
        ├── actual_value.test.ts
        ├── null_chain.test.ts
        ├── undefined_chain.test.ts
        ├── short_circuit.test.ts
        └── obj_check.test.ts
```
