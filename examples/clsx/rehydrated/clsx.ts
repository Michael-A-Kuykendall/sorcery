/**
 * clsx - Rehydrated from Sorcery spells
 * 
 * Build className strings from mixed inputs.
 */

/**
 * #Spell: ToVal
 * ^ Internal: convert a single mixed value to string
 * 
 * ! handles_string
 * ! handles_number
 * ! handles_array_recursively  
 * ! handles_object_by_truthy_keys
 * ! returns_empty_for_falsy
 */
function toVal(mix: any): string {
  let str = '';
  
  // ! handles_string, ! handles_number
  if (typeof mix === 'string' || typeof mix === 'number') {
    str += mix;
  } 
  // ! handles_array_recursively, ! handles_object_by_truthy_keys
  else if (typeof mix === 'object') {
    if (Array.isArray(mix)) {
      // ! handles_array_recursively
      for (let i = 0; i < mix.length; i++) {
        if (mix[i]) {
          const y = toVal(mix[i]);
          if (y) {
            if (str) str += ' ';
            str += y;
          }
        }
      }
    } else {
      // ! handles_object_by_truthy_keys
      for (const key in mix) {
        if (mix[key]) {
          if (str) str += ' ';
          str += key;
        }
      }
    }
  }
  // ! returns_empty_for_falsy (implicit - returns '' for null/undefined/false)
  
  return str;
}

/**
 * #Spell: Clsx
 * ^ Intent: build className strings from mixed inputs, 
 *           filtering falsy values and expanding objects/arrays
 * 
 * ! accepts_variadic_arguments
 * ! returns_space_separated_string
 * ! filters_falsy_values
 * ! strings_pass_through
 * ! numbers_pass_through
 * ! objects_include_truthy_keys
 * ! arrays_recurse_and_flatten
 * ! empty_input_returns_empty_string
 * - includes_falsy_keys_from_objects
 * - includes_null_or_undefined
 */
export function clsx(...args: any[]): string {
  let str = '';
  
  for (let i = 0; i < args.length; i++) {
    const tmp = args[i];
    // ! filters_falsy_values
    if (tmp) {
      const x = toVal(tmp);
      if (x) {
        if (str) str += ' ';
        str += x;
      }
    }
  }
  
  // ! returns_space_separated_string
  // ! empty_input_returns_empty_string
  return str;
}

export default clsx;
