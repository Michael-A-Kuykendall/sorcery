# fast-deep-equal → Sorcery

[fast-deep-equal](https://github.com/epoberezkin/fast-deep-equal) by Evgeny Poberezkin — the fastest deep equality check.

## What It Does

Recursively compare two values for deep equality:

```js
equal({a: {b: 1}}, {a: {b: 1}}) // true
equal([1, 2, 3], [1, 2, 3])     // true
equal(NaN, NaN)                  // true (unlike ===)
```

## The Experiment

Dehydrate fast-deep-equal into spells, then rehydrate without looking at the original.

## Complexity Note

This is the most complex example. It has many edge cases around type handling.
