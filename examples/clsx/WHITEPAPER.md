# Sorcery Case Study: clsx (Reset to Test-Bound Spells)

**Authorship note:** This analysis reflects the Sorcery reset to test-bound spells, emphasizing executable evidence over semantic analysis.

## Abstract

Dehydrated [clsx](https://github.com/lukeed/clsx) into test-bound spells with $ obligations (require/forbid/prove bound to tests), then rehydrated with comprehensive tests to ensure fidelity.

**Result:** Test-evidence guarantees correctness, fail-closed prevents gaps.

---

## The Subject

`clsx` builds className strings from mixed inputs: strings, numbers, objects (truthy keys), arrays (recursive), filtering all falsy values.

## Original Experiment (Semantic Analysis)

**Authorship note:** Original rehydration produced by **Opus Claude 4.5**, using ! guarantees and - exclusions.

Dehydrated clsx into spells, rehydrated without referencing the original.

**Result:** 100% behavioral fidelity.

### Comparison (Original vs Rehydrated)

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

**Behavioral:** 100% match  
**Implementation:** Minor idiom differences (short-circuit vs explicit if, var vs let).

### What the Spell Captured
- ✓ All input types handled
- ✓ Recursive array handling
- ✓ Object key filtering by truthiness
- ✓ Space-joined output
- ✓ Falsy filtering

### What the Spell Missed
Nothing. ToVal helper captured as sub-component.

### Doctrine Observation
Recursive structures naturally captured by "recursively"—no depth specification needed.

---

## New Experiment (Test-Bound Spells)

Rebuilt from scratch using only the test-bound spell notation, with obligations enforced by tests.

**Result:** Identical code to original rehydration, but with executable test suite ensuring fidelity.

### Comparison (New Rehydration vs Original)

Since the functional obligations are identical, the rehydrated code is the same as the original experiment. Differences are in the notation and enforcement:

| Aspect | Original Rehydration | New Rehydration | Verdict |
|--------|----------------------|-----------------|---------|
| Notation | ! guarantees, - exclusions | $ obligations with test bindings | ✅ Improved |
| Enforcement | Semantic interpretation | Test execution + code checks | ✅ Stronger |
| Correctness Guarantee | AI self-check | Fail-closed on missing tests | ✅ Safer |
| Test Suite | None | Comprehensive per obligation | ✅ Added |
| Code Output | Identical | Identical | ✅ Consistent |

**Behavioral:** 100% match to original clsx.  
**Enforcement:** Test-evidence prevents gaps.

### What the Test-Bound Spell Adds
- Executable tests for every prove obligation.
- Fail-closed verification (unsupported obligations = FAIL).
- Clear separation of require (existence), forbid (prohibition), prove (evidence).

### Learnings from Reset
- **Semantic analysis is insufficient:** Original relied on AI interpreting guarantees, risking hallucinations. Test-bound requires runtime evidence.
- **Obligations must be complete:** $ require/forbid/prove cover all aspects; no implicit assumptions.
- **Test bindings enable precision:** Each prove links to a specific test, making specs enforceable.
- **Fail-closed prevents overreach:** Unknown obligations fail immediately, avoiding false positives.
- **Notation simplicity preserved:** $ symbols are as concise as !/-, but more powerful.
- **Rehydration fidelity maintained:** Same code output, but now verifiable.

### Doctrine Refinement
Sorcery is a "napkin shorthand" for intent dehydration, but test-bound makes it "doable" for real enforcement. Spells are complete only with tests; semantic inference is deprecated. This positions Glyph as a language for AI-assisted development with perfect gates.

---

## What the Test-Bound Spell Captures

- ✓ All input types handled (tested)
- ✓ Recursive array handling (tested)
- ✓ Object key filtering (tested)
- ✓ Space-joined output (tested)
- ✓ Falsy filtering (tested)

---

## Doctrine Observation

**Test bindings make spells enforceable:** Every $ prove must have a corresponding test, run by the verifier. No tests = FAIL.

---

## Files

```
examples/clsx/
├── README.md
├── WHITEPAPER.md
├── spells/
│   └── clsx.spell  # Test-bound version
├── rehydrated/
│   └── clsx.ts     # Original semantic rehydration
└── rehydrated2/
    ├── clsx.ts     # New test-bound rehydration (identical code)
    └── tests/      # Test suite
```
