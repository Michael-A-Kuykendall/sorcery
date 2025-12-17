/**
 * fast-deep-equal - Rehydrated from Sorcery spells
 * 
 * Recursively compare two values for structural equality,
 * handling JavaScript types correctly.
 */

/**
 * @Equal
 * : (a, b) -> boolean
 * ! same_reference_returns_true_immediately
 * ! both_must_be_objects_to_continue
 * ! null_not_equal_to_object
 * ! constructor_must_match
 * ! arrays_compared_by_length_then_index
 * ! objects_compared_by_key_count_then_values
 * ! key_order_does_not_matter
 * ! recursively_compares_nested_values
 * ! Date_compared_by_getTime
 * ! RegExp_compared_by_source_and_flags
 * ! NaN_equals_NaN
 * ! primitives_compared_by_strict_equality
 * - handles_circular_references
 * - handles_Map
 * - handles_Set
 * - handles_TypedArray
 * - handles_ArrayBuffer
 * - handles_Symbol_properties
 * - handles_getter_traps
 * ~ no_circular_references_in_input
 * ~ objects_are_plain_no_getters
 * ~ prototype_chain_is_clean
 */
export function equal(a: any, b: any): boolean {
  // ! same_reference_returns_true_immediately
  if (a === b) return true;

  // ! both_must_be_objects_to_continue
  if (a && b && typeof a === 'object' && typeof b === 'object') {
    // ! null_not_equal_to_object (null is typeof 'object' but fails the truthy check above)
    
    // ! constructor_must_match
    if (a.constructor !== b.constructor) return false;

    // ! arrays_compared_by_length_then_index
    if (Array.isArray(a)) {
      const length = a.length;
      if (length !== b.length) return false;
      // ! recursively_compares_nested_values
      for (let i = length; i-- !== 0;) {
        if (!equal(a[i], b[i])) return false;
      }
      return true;
    }

    // ! RegExp_compared_by_source_and_flags
    if (a.constructor === RegExp) {
      return a.source === b.source && a.flags === b.flags;
    }

    // ! Date_compared_by_getTime
    if (a.constructor === Date) {
      return a.getTime() === b.getTime();
    }

    // ~ objects_are_plain_no_getters (we access properties directly)
    // ~ prototype_chain_is_clean (we trust constructor check)

    // ! objects_compared_by_key_count_then_values
    const keys = Object.keys(a);
    const length = keys.length;
    if (length !== Object.keys(b).length) return false;

    // Check all keys exist in b
    for (let i = length; i-- !== 0;) {
      if (!Object.prototype.hasOwnProperty.call(b, keys[i])) return false;
    }

    // ! key_order_does_not_matter
    // ! recursively_compares_nested_values
    for (let i = length; i-- !== 0;) {
      const key = keys[i];
      if (!equal(a[key], b[key])) return false;
    }

    return true;
  }

  // ! NaN_equals_NaN
  // ! primitives_compared_by_strict_equality (handled by === above)
  return a !== a && b !== b;
}

/**
 * @ES6Equal
 * : (a, b) -> boolean
 * > @Equal
 * ! handles_Map_by_size_then_entries
 * ! handles_Set_by_size_then_has
 * ! handles_TypedArray_by_length_then_elements
 * ! Map_keys_compared_by_reference
 * ! Map_values_compared_recursively
 * ! Set_elements_checked_via_has_method
 * ! TypedArray_requires_matching_constructor
 * - handles_circular_references
 * - handles_WeakMap
 * - handles_WeakRef
 * ~ set_has_uses_reference_equality
 * ~ map_get_uses_reference_equality
 * ~ objects_inside_Set_not_deep_compared
 */
export function equalES6(a: any, b: any): boolean {
  // ! same_reference_returns_true_immediately
  if (a === b) return true;

  if (a && b && typeof a === 'object' && typeof b === 'object') {
    // ! constructor_must_match
    if (a.constructor !== b.constructor) return false;

    // ! arrays_compared_by_length_then_index
    if (Array.isArray(a)) {
      const length = a.length;
      if (length !== b.length) return false;
      for (let i = length; i-- !== 0;) {
        if (!equalES6(a[i], b[i])) return false;
      }
      return true;
    }

    // ! handles_Map_by_size_then_entries
    // ! Map_keys_compared_by_reference
    // ! Map_values_compared_recursively
    // ~ map_get_uses_reference_equality
    if (a instanceof Map) {
      if (a.size !== b.size) return false;
      for (const [key, val] of a) {
        // Keys compared by reference (Map.get uses ===)
        if (!b.has(key)) return false;
        if (!equalES6(val, b.get(key))) return false;
      }
      return true;
    }

    // ! handles_Set_by_size_then_has
    // ! Set_elements_checked_via_has_method
    // ~ set_has_uses_reference_equality
    // ~ objects_inside_Set_not_deep_compared
    if (a instanceof Set) {
      if (a.size !== b.size) return false;
      for (const val of a) {
        // Set.has uses reference equality - objects won't match unless same ref
        if (!b.has(val)) return false;
      }
      return true;
    }

    // ! handles_TypedArray_by_length_then_elements
    // ! TypedArray_requires_matching_constructor
    if (ArrayBuffer.isView(a) && ArrayBuffer.isView(b)) {
      const ta = a as unknown as { length: number; [i: number]: number };
      const tb = b as unknown as { length: number; [i: number]: number };
      const length = ta.length;
      if (length !== tb.length) return false;
      for (let i = length; i-- !== 0;) {
        if (ta[i] !== tb[i]) return false;
      }
      return true;
    }

    // ! RegExp_compared_by_source_and_flags
    if (a.constructor === RegExp) {
      return a.source === b.source && a.flags === b.flags;
    }

    // ! Date_compared_by_getTime
    if (a.constructor === Date) {
      return a.getTime() === b.getTime();
    }

    // ! objects_compared_by_key_count_then_values
    const keys = Object.keys(a);
    const length = keys.length;
    if (length !== Object.keys(b).length) return false;

    for (let i = length; i-- !== 0;) {
      if (!Object.prototype.hasOwnProperty.call(b, keys[i])) return false;
    }

    // ! key_order_does_not_matter
    for (let i = length; i-- !== 0;) {
      const key = keys[i];
      if (!equalES6(a[key], b[key])) return false;
    }

    return true;
  }

  // ! NaN_equals_NaN
  return a !== a && b !== b;
}
