// Rehydrated from Sorcery Doctrine spell notation.

type ClassValue =
	| string
	| number
	| null
	| undefined
	| boolean
	| ClassDictionary
	| ClassArray;

interface ClassDictionary {
	[key: string]: any;
}

interface ClassArray extends Array<ClassValue> {}

function toVal(mixed: ClassValue): string {
	if (mixed === null || mixed === undefined || mixed === false || mixed === '') return '';

	if (typeof mixed === 'string') return mixed;
	if (typeof mixed === 'number') return String(mixed);

	if (Array.isArray(mixed)) {
		let out = '';
		for (let i = 0; i < mixed.length; i++) {
			const val = toVal(mixed[i]);
			if (!val) continue;
			out += out ? ' ' + val : val;
		}
		return out;
	}

	if (typeof mixed === 'object') {
		let out = '';
		for (const key in mixed) {
			if ((mixed as any)[key]) {
				out += out ? ' ' + key : key;
			}
		}
		return out;
	}

	return '';
}

export default function clsx(...args: ClassValue[]): string {
	let out = '';
	for (let i = 0; i < args.length; i++) {
		const val = toVal(args[i]);
		if (!val) continue;
		out += out ? ' ' + val : val;
	}
	return out;
}
