# Mitt → Sorcery

This directory maps [mitt](https://github.com/developit/mitt) (a ~200 byte event emitter by Jason Miller) to Sorcery spells.

## The Experiment

1. **Dehydrate:** Read mitt's source, extract the intent, encode as spells
2. **Rehydrate:** Use only the spells to rebuild mitt from scratch
3. **Compare:** See how much survives the round-trip

## Spell Structure

```
mitt/
├── mitt.spell           # Top-level composed spell (the full lib)
├── handler_map.spell    # The internal data structure
├── on.spell             # Subscribe to events
├── off.spell            # Unsubscribe from events
└── emit.spell           # Dispatch events
```

## Original Source

~70 lines of TypeScript. Three methods. One Map.

## What We're Testing

Can the spells capture enough intent that an agent could rebuild mitt without seeing the original code—and produce something functionally equivalent?
