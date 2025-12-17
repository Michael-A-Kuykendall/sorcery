/**
 * fast-deep-equal - Rehydrated from Sorcery spells
 * 
 * Recursively compare two values for deep equality.
 */

/**
 * #Spell: Equal
 * ^ Intent: recursively compare two values for structural equality
 * 
 * ! same_reference_returns_true
 * ! both_must_be_same_type
 * ! constructor_must_match
 * ! primitives_compared_by_value
 * ! NaN_equals_NaN
 * ! positive_zero_equals_negative_zero
 * ! arrays_compared_by_index
 * ! arrays_must_match_length
 * ! objects_compared_by_keys
 * ! objects_must_match_key_count
 * ! key_order_does_not_matter
 * ! recursively_compares_nested
 * ! Date_compared_by_valueOf
 * ! RegExp_compared_by_source_and_flags
 * ! functions_compared_by_reference
 * ! null_only_equals_null
 * ! undefined_only_equals_undefined
 * ! objects_with_custom_valueOf_compared_by_valueOf
 * ! objects_with_custom_toString_compared_by_toString
 * - handles_circular_references
 * - handles_Map
 * - handles_Set
 * - handles_TypedArray
 */
export function equal(a: any, b: any): boolean {
  // ! same_reference_returns_true
  if (a === b) return true;

  // Handle objects (including arrays, dates, etc.)
  if (a && b && typeof a === 'object' && typeof b === 'object') {
    // ! constructor_must_match
    if (a.constructor !== b.constructor) return false;

    // ! arrays_compared_by_index, ! arrays_must_match_length
    if (Array.isArray(a)) {
      const length = a.length;
      if (length !== b.length) return false;
      for (let i = length; i-- !== 0;) {
        // ! recursively_compares_nested
        if (!equal(a[i], b[i])) return false;
      }
      return true;
    }

    // ! RegExp_compared_by_source_and_flags
    if (a.constructor === RegExp) {
      return a.source === b.source && a.flags === b.flags;
    }

    // ! Date_compared_by_valueOf
    // ! objects_with_custom_valueOf_compared_by_valueOf
    if (a.valueOf !== Object.prototype.valueOf) {
      return a.valueOf() === b.valueOf();
    }

    // ! objects_with_custom_toString_compared_by_toString
    if (a.toString !== Object.prototype.toString) {
      return a.toString() === b.toString();
    }

    // ! objects_compared_by_keys, ! objects_must_match_key_count
    const keys = Object.keys(a);
    const length = keys.length;
    if (length !== Object.keys(b).length) return false;

    // Check all keys exist in b
    for (let i = length; i-- !== 0;) {
      if (!Object.prototype.hasOwnProperty.call(b, keys[i])) return false;
    }

    // ! key_order_does_not_matter, ! recursively_compares_nested
    for (let i = length; i-- !== 0;) {
      const key = keys[i];
      if (!equal(a[key], b[key])) return false;
    }

    return true;
  }

  // ! NaN_equals_NaN (NaN !== NaN, so if both are NaN, a !== a && b !== b)
  // ! primitives_compared_by_value (already handled by === above)
  return a !== a && b !== b;
}

/**
 * #Spell: ES6Equal
 * ^ Intent: equal with ES6 Map, Set, and TypedArray support
 * 
 * ! handles_Map_by_size_and_entries
 * ! handles_Set_by_size_and_has
 * ! handles_TypedArray_by_constructor_and_elements
 * ! Map_entries_compared_recursively
 * ! Set_membership_checked_by_has
 * ! TypedArray_requires_same_constructor
 * - handles_circular_references
 */
export function equalES6(a: any, b: any): boolean {
  // ! same_reference_returns_true
  if (a === b) return true;

  if (a && b && typeof a === 'object' && typeof b === 'object') {
    // ! constructor_must_match
    if (a.constructor !== b.constructor) return false;

    // ! arrays_compared_by_index
    if (Array.isArray(a)) {
      const length = a.length;
      if (length !== b.length) return false;
      for (let i = length; i-- !== 0;) {
        if (!equalES6(a[i], b[i])) return false;
      }
      return true;
    }

    // ! handles_Map_by_size_and_entries
    if (a instanceof Map && b instanceof Map) {
      if (a.size !== b.size) return false;
      // Check all keys exist
      for (const [key] of a) {
        if (!b.has(key)) return false;
      }
      // ! Map_entries_compared_recursively
      for (const [key, val] of a) {
        if (!equalES6(val, b.get(key))) return false;
      }
      return true;
    }

    // ! handles_Set_by_size_and_has
    if (a instanceof Set && b instanceof Set) {
      if (a.size !== b.size) return false;
      // ! Set_membership_checked_by_has
      for (const val of a) {
        if (!b.has(val)) return false;
      }
      return true;
    }

    // ! handles_TypedArray_by_constructor_and_elements
    // ! TypedArray_requires_same_constructor
    if (ArrayBuffer.isView(a) && ArrayBuffer.isView(b)) {
      const length = (a as any).length;
      if (length !== (b as any).length) return false;
      for (let i = length; i-- !== 0;) {
        if ((a as any)[i] !== (b as any)[i]) return false;
      }
      return true;
    }

    // ! RegExp_compared_by_source_and_flags
    if (a.constructor === RegExp) {
      return a.source === b.source && a.flags === b.flags;
    }

    // ! Date_compared_by_valueOf, ! objects_with_custom_valueOf_compared_by_valueOf
    if (a.valueOf !== Object.prototype.valueOf) {
      return a.valueOf() === b.valueOf();
    }

    // ! objects_with_custom_toString_compared_by_toString
    if (a.toString !== Object.prototype.toString) {
      return a.toString() === b.toString();
    }

    // Plain objects
    const keys = Object.keys(a);
    const length = keys.length;
    if (length !== Object.keys(b).length) return false;

    for (let i = length; i-- !== 0;) {
      if (!Object.prototype.hasOwnProperty.call(b, keys[i])) return false;
    }

    for (let i = length; i-- !== 0;) {
      const key = keys[i];
      if (!equalES6(a[key], b[key])) return false;
    }

    return true;
  }

  // ! NaN_equals_NaN
  return a !== a && b !== b;
}

export default equal;
