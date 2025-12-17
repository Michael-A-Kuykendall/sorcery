# Sorcery Case Study: ms (rehydrated2)

**Authorship note:** This rehydration + analysis was produced by **GPT-5.2**.

## Abstract

Using only [examples/ms/spells/ms.spell](examples/ms/spells/ms.spell), I rehydrated `ms` into [examples/ms/rehydrated2/ms.ts](examples/ms/rehydrated2/ms.ts) without consulting any upstream source.

**Result:** All spell-stated runtime semantics implemented.

---

## The Subject

`ms` converts:
- human-readable durations ("2 days", "1h", "-3.5w") → millisecond numbers
- millisecond numbers → short or long strings

---

## Spells Used

| Spell | Purpose |
|------:|---------|
| [ms.spell](examples/ms/spells/ms.spell) | Parse + format + constants + options |

---

## Comparison (Spell Intent → Rehydrated2)

| Aspect | Spell intent | Rehydrated2 | Verdict |
|--------|-------------|-------------|---------|
| String → number | `! string_input_returns_number` | `ms("1h") -> number` | ✅ |
| Number → string | `! number_input_returns_string` | `ms(60000) -> "1m"` | ✅ |
| Throws on invalid type | `! throws_on_invalid_type` | throws `TypeError` | ✅ |
| Parse: regex-based | `! regex_based_parsing` | numeric+unit regex | ✅ |
| Unit aliases | `! supports_*` set | supports y/mo/w/d/h/m/s/ms + aliases | ✅ |
| Case-insensitive | `! case_insensitive` | `toLowerCase()` | ✅ |
| Decimal + negative | `! allows_decimal_values`, `! allows_negative_values` | `parseFloat` + `-?` regex | ✅ |
| Space between num/unit | `! allows_space_between_number_and_unit` | `\s*` | ✅ |
| No unit defaults to ms | `! no_unit_defaults_to_milliseconds` | returns raw number | ✅ |
| Unparseable returns NaN | `! returns_NaN_for_unparseable` | returns `NaN` | ✅ |
| Empty string throws | `! throws_on_empty_string` | throws | ✅ |
| >100 chars throws | `! throws_on_string_over_100_chars` | throws | ✅ |
| Long option | `! options_long_selects_verbose_format` | `options.long` | ✅ |
| Plural rule | `! pluralizes_when_value_gte_1_5` | plural when abs/unit >= 1.5 | ✅ |
| Short format rounds | `! rounds_to_integer` | `Math.round` | ✅ |
| Exclusions | `- supports_compound_expressions` | no compound parsing | ✅ |

---

## Notes From This Rehydration

- The spell doesn’t pin an exact regex; the implementation uses a single, simple pattern to satisfy the stated parse guarantees and exclusions.

---

## Files

- Implementation: [examples/ms/rehydrated2/ms.ts](examples/ms/rehydrated2/ms.ts)
- Spell: [examples/ms/spells/ms.spell](examples/ms/spells/ms.spell)
