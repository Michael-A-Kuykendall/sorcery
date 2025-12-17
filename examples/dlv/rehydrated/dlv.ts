/**
 * dlv - Rehydrated from Sorcery spells
 * 
 * Safely traverse nested object properties by path.
 */

/**
 * #Spell: Dlv
 * ^ Intent: safely traverse nested object properties by path, 
 *           returning a default if any segment is missing
 * 
 * ! accepts_string_path_with_dots
 * ! accepts_array_path
 * ! splits_string_on_dots
 * ! returns_undefined_if_no_default_and_missing
 * ! returns_default_if_path_missing
 * ! handles_null_in_chain
 * ! short_circuits_on_nullish
 * - throws_on_missing
 */
export default function dlv(
  obj: any,
  path: string | string[],
  defaultValue?: any
): any {
  // ! accepts_string_path_with_dots, ! splits_string_on_dots
  // ! accepts_array_path (pass through if already array)
  const keys = typeof path === 'string' ? path.split('.') : path;
  
  let result = obj;
  
  for (let i = 0; i < keys.length; i++) {
    // ! handles_null_in_chain, ! handles_undefined_in_chain
    // ! short_circuits_on_nullish
    if (result == null) {
      result = undefined;
      break;
    }
    result = result[keys[i]];
  }
  
  // ! returns_default_if_path_missing
  // ! returns_undefined_if_no_default_and_missing
  return result === undefined ? defaultValue : result;
}
