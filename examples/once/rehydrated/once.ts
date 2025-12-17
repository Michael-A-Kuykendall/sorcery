/**
 * once - Rehydrated from Sorcery spells
 * 
 * Wrap a function so it executes at most once.
 */

interface OnceFunction<T extends (...args: any[]) => any> {
  (...args: Parameters<T>): ReturnType<T>;
  called: boolean;
  value: ReturnType<T> | undefined;
}

interface OnceStrictFunction<T extends (...args: any[]) => any> {
  (...args: Parameters<T>): ReturnType<T>;
  called: boolean;
  value: ReturnType<T> | undefined;
  onceError: string;
}

/**
 * #Spell: Once
 * ^ Intent: wrap a function so it executes at most once, 
 *           caching and returning the result on subsequent calls
 * 
 * ! returns_wrapper_function
 * ! first_call_executes_original
 * ! subsequent_calls_return_cached_value
 * ! preserves_this_context
 * ! preserves_arguments
 * ! sets_called_flag_on_wrapper
 * ! sets_value_property_on_wrapper
 * ! called_flag_initially_false
 * - throws_on_subsequent_calls
 */
export function once<T extends (...args: any[]) => any>(fn: T): OnceFunction<T> {
  const f = function(this: any, ...args: Parameters<T>): ReturnType<T> {
    // ! subsequent_calls_return_cached_value
    if (f.called) return f.value as ReturnType<T>;
    
    // ! first_call_executes_original
    // ! sets_called_flag_on_wrapper
    f.called = true;
    
    // ! preserves_this_context, ! preserves_arguments
    // ! sets_value_property_on_wrapper
    return f.value = fn.apply(this, args);
  } as OnceFunction<T>;
  
  // ! called_flag_initially_false
  f.called = false;
  f.value = undefined;
  
  return f;
}

/**
 * #Spell: OnceStrict
 * ^ Intent: like once, but throws on subsequent calls
 * 
 * ! throws_error_on_subsequent_calls
 * ! error_message_includes_function_name
 * ! uses_fn_name_or_default_string
 * - returns_cached_value_on_subsequent_calls
 */
export function onceStrict<T extends (...args: any[]) => any>(fn: T): OnceStrictFunction<T> {
  const f = function(this: any, ...args: Parameters<T>): ReturnType<T> {
    // ! throws_error_on_subsequent_calls
    if (f.called) throw new Error(f.onceError);
    
    f.called = true;
    return f.value = fn.apply(this, args);
  } as OnceStrictFunction<T>;
  
  // ! uses_fn_name_or_default_string
  const name = fn.name || 'Function wrapped with `once`';
  
  // ! error_message_includes_function_name
  f.onceError = name + " shouldn't be called more than once";
  f.called = false;
  f.value = undefined;
  
  return f;
}

export default once;
