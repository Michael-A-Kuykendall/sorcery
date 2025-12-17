# clsx → Sorcery

[clsx](https://github.com/lukeed/clsx) by Luke Edwards — conditional className builder in ~30 lines.

## What It Does

Construct className strings from various input types:

```js
clsx('foo', 'bar')                    // 'foo bar'
clsx('foo', { bar: true, baz: false}) // 'foo bar'
clsx(['foo', { bar: true }])          // 'foo bar'
clsx('foo', null, undefined, 'bar')   // 'foo bar'
```

## The Experiment

Dehydrate clsx into spells, then rehydrate without looking at the original.
