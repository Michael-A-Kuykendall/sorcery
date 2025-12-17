# Sorcery Case Study: fast-deep-equal (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only [examples/fast-deep-equal/spells/equal.spell](examples/fast-deep-equal/spells/equal.spell), I rehydrated deep equality into [examples/fast-deep-equal/rehydrated2/equal.ts](examples/fast-deep-equal/rehydrated2/equal.ts) without consulting any upstream source.

**Result:** All spell-stated behaviors implemented for both `equal` and `es6Equal`.

---

## The Subject

`fast-deep-equal` recursively compares two values for structural equality across common JavaScript types.

This rehydration provides:
- `default export function equal(a,b)` — base behavior (no Map/Set/TypedArray)
- `export function es6Equal(a,b)` — ES6 behavior (Map/Set/TypedArray)

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [equal.spell](examples/fast-deep-equal/spells/equal.spell) | Base + ES6 variant contracts |

---

## Comparison (Spell Intent → Rehydrated2)

### Base `equal(a, b)`

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| Reference fast path | `! same_reference_returns_true_immediately` | `if (a === b) return true` | ✅ |
| NaN equals NaN | `! NaN_equals_NaN` | `a!==a && b!==b` | ✅ |
| Requires objects to continue | `! both_must_be_objects_to_continue` | non-objects → false | ✅ |
| Constructor match | `! constructor_must_match` | checks constructors | ✅ |
| Arrays by length+index | `! arrays_compared_by_length_then_index` | length then recursion | ✅ |
| Objects by keys+values | `! objects_compared_by_key_count_then_values` | key count then recursion | ✅ |
| Date by time | `! Date_compared_by_getTime` | `getTime()` | ✅ |
| RegExp by source+flags | `! RegExp_compared_by_source_and_flags` | compares both | ✅ |
| Exclusions | no Map/Set/TypedArray/cycles | not implemented | ✅ |

### ES6 `es6Equal(a, b)`

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| Map by size then entries | `! handles_Map_by_size_then_entries` | size then iterates entries | ✅ |
| Map keys by reference | `! Map_keys_compared_by_reference` | `has(key)` | ✅ |
| Map values recursive | `! Map_values_compared_recursively` | recursive on `get(key)` | ✅ |
| Set by size then has | `! handles_Set_by_size_then_has` | size then `has(val)` | ✅ |
| Set semantics assumption | `~ objects_inside_Set_not_deep_compared` | uses `has()` (reference for objects) | ✅ |
| TypedArray by len/elements | `! handles_TypedArray_by_length_then_elements` | checks constructor, len, indices | ✅ |
| TypedArray ctor match | `! TypedArray_requires_matching_constructor` | enforced | ✅ |
| Exclusions | no cycles/WeakMap/WeakRef | not implemented | ✅ |

---

## Notes From This Rehydration

- The ES6 spell explicitly encodes the “Set.has uses reference equality” assumption; the implementation follows that by design.

---

## Files

- Implementation: [examples/fast-deep-equal/rehydrated2/equal.ts](examples/fast-deep-equal/rehydrated2/equal.ts)
- Spell: [examples/fast-deep-equal/spells/equal.spell](examples/fast-deep-equal/spells/equal.spell)
