/**
 * ms - Rehydrated from Sorcery spells
 * 
 * Convert time strings to milliseconds and vice versa.
 */

/**
 * #Spell: TimeConstants
 * ! second_1000
 * ! minute_60_seconds
 * ! hour_60_minutes
 * ! day_24_hours
 * ! week_7_days
 * ! year_365_25_days
 * ! month_year_div_12
 */
const s = 1000;
const m = s * 60;
const h = m * 60;
const d = h * 24;
const w = d * 7;
const y = d * 365.25;
const mo = y / 12;

interface Options {
  long?: boolean;
}

/**
 * #Spell: Parse
 * ^ Intent: parse time string to milliseconds
 * 
 * ! regex_based_parsing
 * ! supports_years_yrs_yr_y
 * ! supports_months_month_mo
 * ! supports_weeks_week_w
 * ! supports_days_day_d
 * ! supports_hours_hrs_hr_h
 * ! supports_minutes_mins_min_m
 * ! supports_seconds_secs_sec_s
 * ! supports_milliseconds_msecs_msec_ms
 * ! case_insensitive
 * ! allows_decimal_values
 * ! allows_negative_values
 * ! allows_space_between_number_and_unit
 * ! no_unit_defaults_to_milliseconds
 * ! returns_NaN_for_unparseable
 * ! throws_on_empty_string
 * ! throws_on_string_over_100_chars
 * - supports_compound_expressions
 */
export function parse(str: string): number {
  // ! throws_on_empty_string, ! throws_on_string_over_100_chars
  if (typeof str !== 'string' || str.length === 0 || str.length > 100) {
    throw new Error(
      `Value provided to ms.parse() must be a string with length between 1 and 99. value=${JSON.stringify(str)}`
    );
  }

  // ! regex_based_parsing, ! case_insensitive
  // ! allows_decimal_values, ! allows_negative_values
  // ! allows_space_between_number_and_unit
  const match = /^(-?\d*\.?\d+)\s*(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|months?|mo|years?|yrs?|yr?|y)?$/i.exec(str);

  if (!match) {
    // ! returns_NaN_for_unparseable
    return NaN;
  }

  const n = parseFloat(match[1]);
  // ! no_unit_defaults_to_milliseconds
  const unit = (match[2] || 'ms').toLowerCase();

  // ! supports_years_yrs_yr_y
  // ! supports_months_month_mo
  // ! supports_weeks_week_w
  // ! supports_days_day_d
  // ! supports_hours_hrs_hr_h
  // ! supports_minutes_mins_min_m
  // ! supports_seconds_secs_sec_s
  // ! supports_milliseconds_msecs_msec_ms
  switch (unit) {
    case 'years':
    case 'year':
    case 'yrs':
    case 'yr':
    case 'y':
      return n * y;
    case 'months':
    case 'month':
    case 'mo':
      return n * mo;
    case 'weeks':
    case 'week':
    case 'w':
      return n * w;
    case 'days':
    case 'day':
    case 'd':
      return n * d;
    case 'hours':
    case 'hour':
    case 'hrs':
    case 'hr':
    case 'h':
      return n * h;
    case 'minutes':
    case 'minute':
    case 'mins':
    case 'min':
    case 'm':
      return n * m;
    case 'seconds':
    case 'second':
    case 'secs':
    case 'sec':
    case 's':
      return n * s;
    case 'milliseconds':
    case 'millisecond':
    case 'msecs':
    case 'msec':
    case 'ms':
      return n;
    default:
      return NaN;
  }
}

/**
 * #Spell: FmtShort
 * ^ Intent: format ms to short string
 * 
 * ! returns_largest_fitting_unit
 * ! rounds_to_integer
 * ! uses_abbreviated_units
 * ! unit_hierarchy_y_mo_w_d_h_m_s_ms
 * ! uses_absolute_value_for_comparison
 */
function fmtShort(ms: number): string {
  const msAbs = Math.abs(ms);
  
  if (msAbs >= y) {
    return `${Math.round(ms / y)}y`;
  }
  if (msAbs >= mo) {
    return `${Math.round(ms / mo)}mo`;
  }
  if (msAbs >= w) {
    return `${Math.round(ms / w)}w`;
  }
  if (msAbs >= d) {
    return `${Math.round(ms / d)}d`;
  }
  if (msAbs >= h) {
    return `${Math.round(ms / h)}h`;
  }
  if (msAbs >= m) {
    return `${Math.round(ms / m)}m`;
  }
  if (msAbs >= s) {
    return `${Math.round(ms / s)}s`;
  }
  return `${ms}ms`;
}

/**
 * #Spell: FmtLong
 * ^ Intent: format ms to long string
 * 
 * ! returns_largest_fitting_unit
 * ! rounds_to_integer
 * ! uses_full_unit_names
 * ! pluralizes_when_value_gte_1_5
 * ! unit_hierarchy_y_mo_w_d_h_m_s_ms
 */
function fmtLong(ms: number): string {
  const msAbs = Math.abs(ms);
  
  if (msAbs >= y) {
    return plural(ms, msAbs, y, 'year');
  }
  if (msAbs >= mo) {
    return plural(ms, msAbs, mo, 'month');
  }
  if (msAbs >= w) {
    return plural(ms, msAbs, w, 'week');
  }
  if (msAbs >= d) {
    return plural(ms, msAbs, d, 'day');
  }
  if (msAbs >= h) {
    return plural(ms, msAbs, h, 'hour');
  }
  if (msAbs >= m) {
    return plural(ms, msAbs, m, 'minute');
  }
  if (msAbs >= s) {
    return plural(ms, msAbs, s, 'second');
  }
  return `${ms} ms`;
}

/**
 * Pluralization helper
 * ! pluralizes_when_value_gte_1_5
 */
function plural(ms: number, msAbs: number, n: number, name: string): string {
  const isPlural = msAbs >= n * 1.5;
  return `${Math.round(ms / n)} ${name}${isPlural ? 's' : ''}`;
}

/**
 * #Spell: Format
 * ^ Intent: format milliseconds to string
 * 
 * ! options_long_selects_verbose_format
 * ! default_is_short_format
 * ! throws_on_non_finite_number
 */
export function format(ms: number, options?: Options): string {
  // ! throws_on_non_finite_number
  if (typeof ms !== 'number' || !Number.isFinite(ms)) {
    throw new Error('Value provided to ms.format() must be of type number.');
  }
  
  // ! options_long_selects_verbose_format, ! default_is_short_format
  return options?.long ? fmtLong(ms) : fmtShort(ms);
}

/**
 * #Spell: Ms
 * ^ Intent: bidirectional conversion between ms numbers and time strings
 * 
 * ! string_input_returns_number
 * ! number_input_returns_string
 * ! throws_on_invalid_type
 */
export function ms(value: string, options?: Options): number;
export function ms(value: number, options?: Options): string;
export function ms(value: string | number, options?: Options): number | string {
  // ! string_input_returns_number
  if (typeof value === 'string') {
    return parse(value);
  }
  // ! number_input_returns_string
  if (typeof value === 'number') {
    return format(value, options);
  }
  // ! throws_on_invalid_type
  throw new Error(
    `Value provided to ms() must be a string or number. value=${JSON.stringify(value)}`
  );
}

export default ms;
