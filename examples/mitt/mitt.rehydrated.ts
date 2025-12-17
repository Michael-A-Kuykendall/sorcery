/**
 * Mitt - Rehydrated from Sorcery spells
 * 
 * This implementation was generated using ONLY the .spell files,
 * without reference to the original mitt source code.
 */

export type EventType = string | symbol;
export type Handler<T = unknown> = (event: T) => void;
export type WildcardHandler<T = Record<string, unknown>> = (
  type: keyof T,
  event: T[keyof T]
) => void;

// @HandlerMap: map_based, supports_wildcard_key, handlers_are_arrays
export type EventHandlerMap<Events extends Record<EventType, unknown>> = Map<
  keyof Events | '*',
  Handler<Events[keyof Events]>[] | WildcardHandler<Events>[]
>;

// @Emitter: has_all_property, has_on_method, has_off_method, has_emit_method
// Exclusions: -has_once_method, -has_clear_method, -has_listeners_method
export interface Emitter<Events extends Record<EventType, unknown>> {
  all: EventHandlerMap<Events>;
  on<Key extends keyof Events>(type: Key, handler: Handler<Events[Key]>): void;
  on(type: '*', handler: WildcardHandler<Events>): void;
  off<Key extends keyof Events>(type: Key, handler?: Handler<Events[Key]>): void;
  off(type: '*', handler: WildcardHandler<Events>): void;
  emit<Key extends keyof Events>(type: Key, event?: Events[Key]): void;
}

/**
 * #Spell: Mitt
 * ^ Intent: minimal event emitter factory exposing subscribe, unsubscribe, 
 *           and dispatch with wildcard support
 * 
 * ! factory_function (not class-based)
 * ! accepts_existing_map
 * ! exposes_internal_map
 * - singleton
 * - namespace_pollution
 */
export default function mitt<Events extends Record<EventType, unknown>>(
  all?: EventHandlerMap<Events>
): Emitter<Events> {
  
  // ! accepts_existing_map, ~ map_injected_or_created
  all = all || new Map();

  return {
    // ! exposes_internal_map
    all,

    /**
     * #Spell: On
     * ^ Intent: register an event handler for a given type, 
     *           accumulating multiple handlers per type
     * 
     * ! appends_to_existing
     * ! creates_list_if_missing
     * ! allows_duplicate_handlers
     * - returns_unsubscribe_function
     * - deduplication
     */
    on<Key extends keyof Events>(
      type: Key,
      handler: Handler<Events[Key]> | WildcardHandler<Events>
    ) {
      const handlers = all!.get(type);
      if (handlers) {
        // ! appends_to_existing, ! allows_duplicate_handlers
        handlers.push(handler as Handler<Events[keyof Events]>);
      } else {
        // ! creates_list_if_missing
        all!.set(type, [handler] as Handler<Events[keyof Events]>[]);
      }
    },

    /**
     * #Spell: Off
     * ^ Intent: remove a specific handler or all handlers for a given event type
     * 
     * ! removes_by_reference_equality
     * ! clears_all_if_handler_omitted
     * ! no_op_if_not_found
     * ! safe_on_missing_type
     * - throws_on_missing
     */
    off<Key extends keyof Events>(
      type: Key,
      handler?: Handler<Events[Key]> | WildcardHandler<Events>
    ) {
      const handlers = all!.get(type);
      // ! safe_on_missing_type
      if (handlers) {
        if (handler) {
          // ! removes_by_reference_equality
          const idx = handlers.indexOf(handler as Handler<Events[keyof Events]>);
          if (idx > -1) {
            handlers.splice(idx, 1);
          }
          // ! no_op_if_not_found (implicit - no else)
        } else {
          // ! clears_all_if_handler_omitted
          all!.set(type, []);
        }
      }
    },

    /**
     * #Spell: Emit
     * ^ Intent: invoke all registered handlers for a given event type, 
     *           then wildcard handlers
     * 
     * ! invokes_type_handlers_first
     * ! invokes_wildcard_handlers_second
     * ! handlers_receive_event_payload
     * ! wildcard_handlers_receive_type_and_payload
     * ! iterates_snapshot_of_handlers
     * ! no_op_if_no_handlers
     * - supports_manual_wildcard_emit
     * - async_dispatch
     * - catches_handler_errors
     */
    emit<Key extends keyof Events>(type: Key, event?: Events[Key]) {
      // ! invokes_type_handlers_first
      let handlers = all!.get(type);
      if (handlers) {
        // ! iterates_snapshot_of_handlers (slice creates copy)
        (handlers as Handler<Events[keyof Events]>[])
          .slice()
          .forEach((handler) => {
            // ! handlers_receive_event_payload
            handler(event!);
          });
      }

      // ! invokes_wildcard_handlers_second
      handlers = all!.get('*');
      if (handlers) {
        // ! iterates_snapshot_of_handlers
        (handlers as WildcardHandler<Events>[])
          .slice()
          .forEach((handler) => {
            // ! wildcard_handlers_receive_type_and_payload
            handler(type, event!);
          });
      }
    }
  };
}
