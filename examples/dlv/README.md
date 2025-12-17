# dlv → Sorcery

[dlv](https://github.com/developit/dlv) by Jason Miller — deep object property access in ~20 lines.

## What It Does

Safely access nested object properties using a dot-separated string or array path.

```js
dlv(obj, 'a.b.c')      // obj?.a?.b?.c
dlv(obj, ['a','b'])    // obj?.a?.b
dlv(obj, 'x.y', 'def') // returns 'def' if path not found
```

## The Experiment

Dehydrate dlv into spells, then rehydrate without looking at the original.
