# Sorcery Case Study: mitt (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only the Sorcery spell files under [examples/mitt/spells](examples/mitt/spells), I rehydrated a minimal event emitter into [examples/mitt/rehydrated2/mitt.ts](examples/mitt/rehydrated2/mitt.ts) without consulting any upstream source.

**Result:** All spell-stated behavioral guarantees implemented.

---

## The Subject

**mitt** is a tiny event emitter with three operations:
- `on(type, handler)` — subscribe
- `off(type, handler?)` — unsubscribe (or clear all for a type)
- `emit(type, event?)` — dispatch

Plus wildcard support (`*` receives all events).

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [handler_map.spell](examples/mitt/spells/handler_map.spell) | Storage model for handlers |
| [on.spell](examples/mitt/spells/on.spell) | Subscribe behavior |
| [off.spell](examples/mitt/spells/off.spell) | Unsubscribe / clear behavior |
| [emit.spell](examples/mitt/spells/emit.spell) | Dispatch + wildcard ordering |
| [mitt.spell](examples/mitt/spells/mitt.spell) | Factory + surface API |

---

## Comparison (Spell Intent → Rehydrated2)

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| Factory function | `! factory_function` | `export default function mitt(...)` | ✅ |
| Accept existing map | `! accepts_existing_map` | `all?: HandlerMap` injection | ✅ |
| Map-based storage | `! map_based` | `Map<EventType | '*', Handler[]>` | ✅ |
| Handlers are arrays | `! handlers_are_arrays` | `Array<(...args)=>void>` lists | ✅ |
| Allows duplicates | `! allows_duplicate_handlers` | `push()` no dedupe | ✅ |
| Wildcard key | `! supports_wildcard_key` | uses `'*'` in map | ✅ |
| Emit ordering | `! invokes_type_handlers_first` then wildcard | type first, then `'*'` | ✅ |
| Snapshot iteration | `! iterates_snapshot_of_handlers` | `.slice()` before iterating | ✅ |
| Clear-all when omitted | `! clears_all_if_handler_omitted` | `handlers.length = 0` | ✅ |
| Frozen interface | `! returns_frozen_interface` | `Object.freeze({ ... })` | ✅ |
| Exclusions | no async/once/errors | not implemented | ✅ |

---

## Notes From This Rehydration

- The spells require “snapshot iteration” and explicit wildcard ordering; both are load-bearing for correctness under mutation during `emit()`.
- Types are expressed as a generic event map in TypeScript; runtime behavior stays minimal.

---

## Files

- Implementation: [examples/mitt/rehydrated2/mitt.ts](examples/mitt/rehydrated2/mitt.ts)
- Spells: [examples/mitt/spells](examples/mitt/spells)

---

## Reproduce

1. Read the spells in [examples/mitt/spells](examples/mitt/spells)
2. Inspect the rehydration in [examples/mitt/rehydrated2/mitt.ts](examples/mitt/rehydrated2/mitt.ts)
3. Validate key behaviors manually:
   - wildcard handlers run after typed handlers
   - removing handlers during emit doesn’t break iteration (snapshot)
