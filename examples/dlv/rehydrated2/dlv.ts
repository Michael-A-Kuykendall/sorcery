// Rehydrated from Sorcery Doctrine spell notation.

type PathSegment = string | number;
export type Path = string | ReadonlyArray<PathSegment>;

export default function dlv<TDefault = undefined>(
	obj: unknown,
	path: Path,
	defaultValue?: TDefault,
): unknown | TDefault {
	if (path === '' || (Array.isArray(path) && path.length === 0)) {
		return obj as any;
	}

	const segments: ReadonlyArray<PathSegment> = Array.isArray(path) ? path : path.split('.');

	let current: any = obj;
	for (let i = 0; i < segments.length; i++) {
		if (current == null) {
			return defaultValue as any;
		}

		const key = segments[i] as any;
		if (key in Object(current)) {
			current = current[key];
			continue;
		}

		return defaultValue as any;
	}

	return current as any;
}
