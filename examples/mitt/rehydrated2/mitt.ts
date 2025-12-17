// Rehydrated from Sorcery Doctrine spell notation.

export type EventType = string | symbol;

export type Handler<T = unknown> = (event: T) => void;
export type WildcardHandler<TEventMap extends Record<PropertyKey, unknown> = Record<PropertyKey, unknown>> = (
	type: keyof TEventMap,
	event: TEventMap[keyof TEventMap] | undefined,
) => void;

export type HandlerMap = Map<EventType | '*', Array<(...args: any[]) => void>>;

export interface Emitter<TEventMap extends Record<PropertyKey, unknown> = Record<PropertyKey, unknown>> {
	all: HandlerMap;
	on<TType extends keyof TEventMap>(type: TType, handler: Handler<TEventMap[TType]>): void;
	on(type: '*', handler: WildcardHandler<TEventMap>): void;
	off<TType extends keyof TEventMap>(type: TType, handler?: Handler<TEventMap[TType]>): void;
	off(type: '*', handler?: WildcardHandler<TEventMap>): void;
	emit<TType extends keyof TEventMap>(type: TType, event?: TEventMap[TType]): void;
}

export default function mitt<TEventMap extends Record<PropertyKey, unknown> = Record<PropertyKey, unknown>>(
	all?: HandlerMap,
): Emitter<TEventMap> {
	const map: HandlerMap = all ?? new Map();

	const on: Emitter<TEventMap>['on'] = (type: any, handler: any) => {
		let handlers = map.get(type);
		if (!handlers) {
			handlers = [];
			map.set(type, handlers);
		}
		handlers.push(handler);
	};

	const off: Emitter<TEventMap>['off'] = (type: any, handler?: any) => {
		const handlers = map.get(type);
		if (!handlers) return;

		if (!handler) {
			handlers.length = 0;
			return;
		}

		for (let i = 0; i < handlers.length; i++) {
			if (handlers[i] === handler) {
				handlers.splice(i, 1);
				break;
			}
		}
	};

	const emit: Emitter<TEventMap>['emit'] = (type: any, event?: any) => {
		const typeHandlers = map.get(type);
		if (typeHandlers && typeHandlers.length) {
			for (const handler of typeHandlers.slice()) {
				handler(event);
			}
		}

		if (type === '*') return;

		const wildcardHandlers = map.get('*');
		if (wildcardHandlers && wildcardHandlers.length) {
			for (const handler of wildcardHandlers.slice()) {
				handler(type, event);
			}
		}
	};

	const emitter: Emitter<TEventMap> = Object.freeze({
		all: map,
		on,
		off,
		emit,
	});

	return emitter;
}
