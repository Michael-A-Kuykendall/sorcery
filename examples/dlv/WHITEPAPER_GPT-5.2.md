# Sorcery Case Study: dlv (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only [examples/dlv/spells/dlv.spell](examples/dlv/spells/dlv.spell), I rehydrated `dlv` into [examples/dlv/rehydrated2/dlv.ts](examples/dlv/rehydrated2/dlv.ts) without consulting any upstream source.

**Result:** All spell-stated behavioral guarantees implemented.

---

## The Subject

`dlv` safely traverses nested properties given a dot-path string or path array, returning a default value when traversal fails.

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [dlv.spell](examples/dlv/spells/dlv.spell) | Single function contract + guarantees |

---

## Comparison (Spell Intent → Rehydrated2)

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| String path support | `! accepts_string_path_with_dots` | `path.split('.')` | ✅ |
| Array path support | `! accepts_array_path` | uses array as-is | ✅ |
| Nullish short-circuit | `! short_circuits_on_nullish` | `if (current == null) return default` | ✅ |
| Default if missing | `! returns_default_if_path_missing` | returns `defaultValue` | ✅ |
| Undefined if no default | `! returns_undefined_if_no_default_and_missing` | `defaultValue` omitted → `undefined` | ✅ |
| No throws | `- throws_on_missing` | never throws for missing path | ✅ |
| No mutation | `- modifies_input` | read-only traversal | ✅ |

---

## Notes From This Rehydration

- I added one pragmatic edge-case: empty path (`''` or `[]`) returns the original `obj`. This was not specified in the spell; it does not conflict with any stated guarantees.

---

## Files

- Implementation: [examples/dlv/rehydrated2/dlv.ts](examples/dlv/rehydrated2/dlv.ts)
- Spell: [examples/dlv/spells/dlv.spell](examples/dlv/spells/dlv.spell)
