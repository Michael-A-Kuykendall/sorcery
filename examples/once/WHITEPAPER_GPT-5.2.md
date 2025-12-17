# Sorcery Case Study: once (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only [examples/once/spells/once.spell](examples/once/spells/once.spell), I rehydrated `once` into [examples/once/rehydrated2/once.ts](examples/once/rehydrated2/once.ts) without consulting any upstream source.

**Result:** All spell-stated runtime semantics implemented; function name/length preservation is best-effort (JavaScript limitations).

---

## The Subject

`once(fn)` wraps a function so it executes at most once; later calls return the cached first result. `onceStrict(fn)` throws on subsequent calls.

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [once.spell](examples/once/spells/once.spell) | Once + OnceStrict + wrapper metadata |

---

## Comparison (Spell Intent → Rehydrated2)

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| First call executes original | `! first_call_executes_original` | calls `fn.apply(this, args)` once | ✅ |
| Subsequent returns cached | `! subsequent_calls_return_cached_value` | returns cached `value` | ✅ |
| Preserves `this` | `! preserves_this_context` | uses `apply(this, ...)` | ✅ |
| Preserves first-call args | `! preserves_arguments_on_first_call` | caches result only from first call | ✅ |
| Exposes `.called` | `! exposes_called_flag_on_wrapper` | `.called` property | ✅ |
| Exposes `.value` | `! exposes_value_property_on_wrapper` | `.value` property | ✅ |
| called initially false | `! called_flag_initially_false` | initialized false | ✅ |
| onceStrict throws later | `! throws_error_on_subsequent_calls` | throws Error on 2nd call | ✅ |
| Error includes fn name | `! error_message_includes_function_name` | includes `fn.name` fallback | ✅ |
| Exposes `.onceError` | `! exposes_onceError_property` | sets `.onceError` | ✅ |
| Preserve function length | `! preserves_function_length` | arity wrappers (0–5, else rest) | ⚠️ Best-effort |
| Preserve function name | `! preserves_function_name` | creates named wrapper via computed key | ⚠️ Best-effort |

---

## Notes From This Rehydration

- JavaScript does not guarantee you can perfectly preserve both `name` and `length` across all wrapping strategies; this implementation uses common best-effort techniques (arity-specific wrappers + computed-name wrapper).
- If the first call throws, the wrapper does not mark itself “called”, allowing re-attempts (consistent with “at most once successful execution” semantics and avoids caching failures).

---

## Files

- Implementation: [examples/once/rehydrated2/once.ts](examples/once/rehydrated2/once.ts)
- Spell: [examples/once/spells/once.spell](examples/once/spells/once.spell)
