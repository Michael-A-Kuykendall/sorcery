/**
 * once - Rehydrated from Sorcery spells
 * 
 * Wrap a function so it executes at most once,
 * caching and returning the result on subsequent calls.
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
 * @Once
 * : (fn) -> wrapped_fn
 * ! first_call_executes_original
 * ! subsequent_calls_return_cached_value
 * ! preserves_this_context
 * ! preserves_arguments_on_first_call
 * ! preserves_function_name
 * ! preserves_function_length
 * ! exposes_called_flag_on_wrapper
 * ! exposes_value_property_on_wrapper
 * ! called_flag_initially_false
 * - throws_on_subsequent_calls
 * - clears_cached_value
 * - resets_called_flag
 * ~ fn_is_callable
 */
export function once<T extends (...args: any[]) => any>(fn: T): OnceFunction<T> {
  let called = false;
  let value: ReturnType<T> | undefined;

  // ! preserves_function_name, ! preserves_function_length
  const wrapper = {
    [fn.name || 'once']: function(this: any, ...args: Parameters<T>): ReturnType<T> {
      // ! subsequent_calls_return_cached_value
      if (called) return value as ReturnType<T>;

      // ! first_call_executes_original
      called = true;

      // ! preserves_this_context, ! preserves_arguments_on_first_call
      return value = fn.apply(this, args);
    }
  }[fn.name || 'once'] as OnceFunction<T>;

  // ! preserves_function_length
  Object.defineProperty(wrapper, 'length', { value: fn.length });

  // ! exposes_called_flag_on_wrapper
  Object.defineProperty(wrapper, 'called', {
    get: () => called,
    set: (v) => { called = v; }
  });

  // ! exposes_value_property_on_wrapper
  Object.defineProperty(wrapper, 'value', {
    get: () => value,
    set: (v) => { value = v; }
  });

  return wrapper;
}

/**
 * @OnceStrict
 * : (fn) -> wrapped_fn
 * > @Once
 * ! throws_error_on_subsequent_calls
 * ! error_message_includes_function_name
 * ! uses_fn_name_or_fallback
 * ! exposes_onceError_property
 * - returns_cached_value_on_subsequent_calls
 * - silent_no_op
 */
export function onceStrict<T extends (...args: any[]) => any>(fn: T): OnceStrictFunction<T> {
  let called = false;
  let value: ReturnType<T> | undefined;

  // ! uses_fn_name_or_fallback
  const name = fn.name || 'Function wrapped with `once`';

  // ! error_message_includes_function_name
  const onceError = `${name} shouldn't be called more than once`;

  // ! preserves_function_name, ! preserves_function_length
  const wrapper = {
    [fn.name || 'onceStrict']: function(this: any, ...args: Parameters<T>): ReturnType<T> {
      // ! throws_error_on_subsequent_calls
      if (called) throw new Error(onceError);

      called = true;
      return value = fn.apply(this, args);
    }
  }[fn.name || 'onceStrict'] as OnceStrictFunction<T>;

  Object.defineProperty(wrapper, 'length', { value: fn.length });

  Object.defineProperty(wrapper, 'called', {
    get: () => called,
    set: (v) => { called = v; }
  });

  Object.defineProperty(wrapper, 'value', {
    get: () => value,
    set: (v) => { value = v; }
  });

  // ! exposes_onceError_property
  Object.defineProperty(wrapper, 'onceError', {
    get: () => onceError
  });

  return wrapper;
}
