# ms → Sorcery

[ms](https://github.com/vercel/ms) by Vercel — convert time strings to milliseconds and vice versa.

## What It Does

Parse human-readable time strings to milliseconds, or format milliseconds to human-readable strings:

```js
ms('2 days')   // 172800000
ms('1h')       // 3600000
ms(60000)      // '1m'
ms(60000, { long: true }) // '1 minute'
```

## The Experiment

Dehydrate ms into spells, then rehydrate without looking at the original.
