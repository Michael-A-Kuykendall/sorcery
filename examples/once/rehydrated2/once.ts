// Rehydrated from Sorcery Doctrine spell notation.

type AnyFn = (this: any, ...args: any[]) => any;

export type OnceWrapped<F extends AnyFn> = F & {
	called: boolean;
	value: ReturnType<F> | undefined;
};

export type OnceStrictWrapped<F extends AnyFn> = OnceWrapped<F> & {
	onceError?: Error;
};

function wrapWithLength(core: AnyFn, length: number): AnyFn {
	switch (length) {
		case 0:
			return function (this: any) {
				return core.apply(this, arguments as any);
			};
		case 1:
			return function (this: any, a: any) {
				return core.apply(this, arguments as any);
			};
		case 2:
			return function (this: any, a: any, b: any) {
				return core.apply(this, arguments as any);
			};
		case 3:
			return function (this: any, a: any, b: any, c: any) {
				return core.apply(this, arguments as any);
			};
		case 4:
			return function (this: any, a: any, b: any, c: any, d: any) {
				return core.apply(this, arguments as any);
			};
		case 5:
			return function (this: any, a: any, b: any, c: any, d: any, e: any) {
				return core.apply(this, arguments as any);
			};
		default:
			return function (this: any, ...args: any[]) {
				return core.apply(this, args);
			};
	}
}

function namedFunction(name: string, fn: AnyFn): AnyFn {
	const key = name && name.length ? name : 'once';
	return { [key]: fn }[key];
}

export default function once<F extends AnyFn>(fn: F): OnceWrapped<F> {
	let called = false;
	let value: ReturnType<F> | undefined;

	const core: AnyFn = function (this: any, ...args: any[]) {
		if (called) return value;

		// Preserve `this` and only preserve arguments from the first call.
		try {
			value = fn.apply(this, args);
			called = true;
			(wrapper as any).called = true;
			(wrapper as any).value = value;
			return value;
		} catch (err) {
			// If the first call throws, allow re-attempt.
			throw err;
		}
	};

	const arityWrapper = wrapWithLength(core, fn.length);
	const wrapper = namedFunction(fn.name, arityWrapper) as OnceWrapped<F>;

	(wrapper as any).called = false;
	(wrapper as any).value = undefined;

	return wrapper;
}

export function onceStrict<F extends AnyFn>(fn: F): OnceStrictWrapped<F> {
	const base = once(fn) as OnceStrictWrapped<F>;
	const original = base as unknown as AnyFn;

	const core: AnyFn = function (this: any, ...args: any[]) {
		if ((base as any).called) {
			const name = fn.name && fn.name.length ? fn.name : 'function';
			const err = new Error(`Function \"${name}\" can only be called once`);
			(base as any).onceError = err;
			throw err;
		}
		return original.apply(this, args);
	};

	const arityWrapper = wrapWithLength(core, fn.length);
	const strictWrapped = namedFunction(fn.name, arityWrapper) as OnceStrictWrapped<F>;

	// Copy the exposed state fields over (they will be mutated by the base wrapper).
	(strictWrapped as any).called = (base as any).called;
	(strictWrapped as any).value = (base as any).value;
	Object.defineProperty(strictWrapped, 'called', {
		get: () => (base as any).called,
		set: (v: boolean) => ((base as any).called = v),
		enumerable: true,
		configurable: true,
	});
	Object.defineProperty(strictWrapped, 'value', {
		get: () => (base as any).value,
		set: (v: any) => ((base as any).value = v),
		enumerable: true,
		configurable: true,
	});
	Object.defineProperty(strictWrapped, 'onceError', {
		get: () => (base as any).onceError,
		set: (v: any) => ((base as any).onceError = v),
		enumerable: true,
		configurable: true,
	});

	return strictWrapped;
}
