# Lessons Learned: Context Asymmetry Is The Point

This repo’s experiments (especially Shimmy) produced a useful, concrete result:

> Sorcery works best as a **downward-facing transmission tool**: a high-context planner **casts** spells; low-context agents **invoke** them.

The biggest failures we observed were not “coding mistakes”; they were **casting mistakes**.

---

## 1) What the experiments actually showed

### What succeeded
- Spells preserved **architecture**: module boundaries, dependency flow, trait/interface boundaries, route shapes, and the “bones” of a system.
- Independent rehydrations converged on the same **core skeleton** (registry/discovery/engine/server/OpenAI compat) even when details varied.

### What failed (pattern)
The most frequent divergences were:
- **Exact inventories drifted**: CLI command sets, server endpoints, TypeScript export surfaces.
- **Production subsystems disappeared** when not explicitly spelled: metrics/observability/caching/workflows/invariants.
- **Protocol and format precision degraded** when only described loosely: manifests/blobs, regex patterns, platform-specific scanning.

Crucially: these failures appeared at the spell level. Once a spell was underspecified, invocation was forced to improvise.

---

## 2) The key asymmetry: casting vs invocation

Sorcery’s doctrine already says:
- Casting destroys excess context.
- Invocation must not invent missing intent.

The experiment adds a pragmatic corollary:

> **Casting is the high-context task. Invocation is the low-context task.**

A low-context agent is incentivized to “make it work” by filling blanks with reasonable defaults. That’s not malice; it’s the only move available when a spell doesn’t seal the edge.

A high-context agent (or a careful human) is more likely to notice:
- “This set is supposed to be exact.”
- “This file name contains two responsibilities; split the spell.”
- “This compatibility layer needs a contract, not just a mention.”

---

## 3) Most ‘gaps’ were not missing-glyph problems

The observed misses can mostly be expressed with the existing glyphs:

- `@` declare the missing subsystem as an entity (so it must exist)
- `:` bind explicit contracts (endpoints, exported symbols, CLI commands)
- `!` enforce **exactness** and invariants
- `-` forbid invention (“no extra commands/endpoints/exports”)
- `>` force dependencies (if Server depends on Observability, it must be implemented)
- `~` state platform/format assumptions explicitly
- `?` block invocation when you aren’t sure what must be true

The failure mode was generally: **we didn’t use the glyphs to seal the ambiguous boundaries**.

---

## 4) The “Exact Set Rule” (the single most important learning)

If a set is intended to be exact, the spell must say so.

Exact sets include:
- CLI commands
- HTTP routes
- exported API surface
- supported backends
- supported compatibility endpoints

**Casting rule:**

```glyph
! inventory_exact
- invented_items
```

If you can’t confidently declare an inventory as exact, you must gate it:

```glyph
? is_inventory_exact
```

This converts the most common “AI filled in the blanks” failure into a slice-gating failure—which is exactly what Sorcery wants.

---

## 5) Practical workflow recommendation

### Recommended roles
- **High-context planner (human or strong model):** casts spells, enforces slice gating, resolves `?`, adds `!/-` for exactness.
- **Low-context invoker (small model, junior, future-you):** implements from sealed spells; does not invent.

### Recommended pipeline
1. Cast spells (high-context)
2. Run a “spell review” pass looking only for: inventories, boundaries, and forbidden invention
3. Invoke (low-context)
4. Compare outputs across independent invocations to locate systematic underspecification
5. Patch spells (not code) when the gap is a casting gap

---

## 6) What this means for Sorcery’s claim

The experiment supports the central claim:

- Sorcery is effective as a **creation and handoff system**.
- It is not intended to be code stenography or byte-for-byte capture.

The system’s success depends on allocating the expensive reasoning to the caster and keeping invocation mechanical.

---

## 7) Related documents

- Shimmy misses/hits: `examples/shimmy/REHYDRATION_FORENSICS.md`
- Shimmy diff summary: `examples/shimmy/DIFF_ANALYSIS.md`
- Rehydrated v1 vs v2 comparison: `examples/REHYDRATION_COMPARISON_V1_V2.md`
- Gap → glyph mapping: `examples/GLYPH_GAP_TO_GLYPH_MAPPING.md`
