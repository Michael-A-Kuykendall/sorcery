// Rehydrated from Sorcery Doctrine spell notation.

export interface MsOptions {
	long?: boolean;
}

const SECOND = 1000;
const MINUTE = 60 * SECOND;
const HOUR = 60 * MINUTE;
const DAY = 24 * HOUR;
const WEEK = 7 * DAY;
const YEAR = 365.25 * DAY;
const MONTH = YEAR / 12;

const PARSE_RE = /^(-?(?:\d+)?\.?\d+)\s*([a-zA-Z]+)?$/;

function parse(str: string): number {
	if (str.length > 100) throw new Error('String exceeds 100 characters');
	if (str.trim().length === 0) throw new Error('Empty string');

	const match = PARSE_RE.exec(str);
	if (!match) return NaN;

	const n = parseFloat(match[1]);
	const unitRaw = match[2];
	if (!unitRaw) return n;

	switch (unitRaw.toLowerCase()) {
		case 'y':
		case 'yr':
		case 'yrs':
		case 'year':
		case 'years':
			return n * YEAR;

		case 'mo':
		case 'month':
		case 'months':
			return n * MONTH;

		case 'w':
		case 'week':
		case 'weeks':
			return n * WEEK;

		case 'd':
		case 'day':
		case 'days':
			return n * DAY;

		case 'h':
		case 'hr':
		case 'hrs':
		case 'hour':
		case 'hours':
			return n * HOUR;

		case 'm':
		case 'min':
		case 'mins':
		case 'minute':
		case 'minutes':
			return n * MINUTE;

		case 's':
		case 'sec':
		case 'secs':
		case 'second':
		case 'seconds':
			return n * SECOND;

		case 'ms':
		case 'msec':
		case 'msecs':
		case 'millisecond':
		case 'milliseconds':
			return n;

		default:
			return NaN;
	}
}

function fmtShort(ms: number): string {
	const abs = Math.abs(ms);
	if (abs >= YEAR) return `${Math.round(ms / YEAR)}y`;
	if (abs >= MONTH) return `${Math.round(ms / MONTH)}mo`;
	if (abs >= WEEK) return `${Math.round(ms / WEEK)}w`;
	if (abs >= DAY) return `${Math.round(ms / DAY)}d`;
	if (abs >= HOUR) return `${Math.round(ms / HOUR)}h`;
	if (abs >= MINUTE) return `${Math.round(ms / MINUTE)}m`;
	if (abs >= SECOND) return `${Math.round(ms / SECOND)}s`;
	return `${Math.round(ms)}ms`;
}

function fmtLong(ms: number): string {
	const abs = Math.abs(ms);

	if (abs >= YEAR) return plural(ms, YEAR, 'year');
	if (abs >= MONTH) return plural(ms, MONTH, 'month');
	if (abs >= WEEK) return plural(ms, WEEK, 'week');
	if (abs >= DAY) return plural(ms, DAY, 'day');
	if (abs >= HOUR) return plural(ms, HOUR, 'hour');
	if (abs >= MINUTE) return plural(ms, MINUTE, 'minute');
	if (abs >= SECOND) return plural(ms, SECOND, 'second');
	return `${Math.round(ms)} ms`;
}

function plural(ms: number, unit: number, name: string): string {
	const val = Math.abs(ms) / unit;
	const rounded = Math.round(ms / unit);
	return `${rounded} ${name}${val >= 1.5 ? 's' : ''}`;
}

function format(ms: number, options?: MsOptions): string {
	if (!Number.isFinite(ms)) throw new TypeError('Expected a finite number');
	return options && options.long ? fmtLong(ms) : fmtShort(ms);
}

export default function ms(value: string, options?: MsOptions): number;
export default function ms(value: number, options?: MsOptions): string;
export default function ms(value: string | number, options?: MsOptions): number | string {
	if (typeof value === 'string') return parse(value);
	if (typeof value === 'number') return format(value, options);
	throw new TypeError('Expected a string or number');
}
