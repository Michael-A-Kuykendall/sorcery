# once → Sorcery

[once](https://github.com/isaacs/once) by Isaac Z. Schlueter — ensure a function is only called once.

## What It Does

Wrap a function so it can only execute once. Subsequent calls return the cached result.

```js
const fn = once(() => Math.random())
fn() // 0.1234
fn() // 0.1234 (same value, function not re-executed)
```

Also provides a strict mode that throws on subsequent calls.

## The Experiment

Dehydrate once into spells, then rehydrate without looking at the original.

## Note

The original `once` depends on `wrappy` for function wrapper preservation. We'll capture the core behavior.
