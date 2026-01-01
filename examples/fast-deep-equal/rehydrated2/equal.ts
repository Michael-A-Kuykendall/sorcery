// Rehydrated from Sorcery Doctrine spell notation.

function isObject(value: any): value is object {
	return value !== null && typeof value === 'object';
}

function isRegExp(value: any): value is RegExp {
	return value instanceof RegExp;
}

function isDate(value: any): value is Date {
	return value instanceof Date;
}

function isTypedArray(value: any): value is
	| Int8Array
	| Uint8Array
	| Uint8ClampedArray
	| Int16Array
	| Uint16Array
	| Int32Array
	| Uint32Array
	| Float32Array
	| Float64Array
	| BigInt64Array
	| BigUint64Array {
	return typeof ArrayBuffer !== 'undefined' && ArrayBuffer.isView(value) && !(value instanceof DataView);
}

export default function equal(a: any, b: any): boolean {
	if (a === b) return true;
	if (a !== a && b !== b) return true;

	if (!isObject(a) || !isObject(b)) return false;
	if (a.constructor !== b.constructor) return false;

	// Functions are never equal
	if (typeof a === 'function' || typeof b === 'function') return false;

	if (isDate(a)) return a.getTime() === (b as Date).getTime();
	if (isRegExp(a)) {
		const rb = b as RegExp;
		return a.source === rb.source && a.flags === rb.flags;
	}

	if (isTypedArray(a)) {
		if (!isTypedArray(b)) return false;
		if (a.constructor !== b.constructor) return false;
		if (a.length !== b.length) return false;
		for (let i = 0; i < a.length; i++) {
			if (a[i] !== b[i]) return false;
		}
		return true;
	}

	if (a instanceof Map) {
		const mb = b as Map<any, any>;
		if (a.size !== mb.size) return false;
		for (const [key, val] of a.entries()) {
			if (!mb.has(key)) return false;
			if (!equal(val, mb.get(key))) return false;
		}
		return true;
	}

	if (a instanceof Set) {
		const sb = b as Set<any>;
		if (a.size !== sb.size) return false;
		for (const val of a.values()) {
			if (!sb.has(val)) return false;
		}
		return true;
	}

	if (Array.isArray(a)) {
		if (!Array.isArray(b)) return false;
		if (a.length !== b.length) return false;
		for (let i = 0; i < a.length; i++) {
			if (!equal(a[i], b[i])) return false;
		}
		return true;
	}

	const aKeys = Object.keys(a);
	const bKeys = Object.keys(b);
	if (aKeys.length !== bKeys.length) return false;

	for (let i = 0; i < aKeys.length; i++) {
		const key = aKeys[i];
		if (!(key in b)) return false;
		if (!equal(a[key], b[key])) return false;
	}

	return true;
}

export function es6Equal(a: any, b: any): boolean {
	return equal(a, b);
}
