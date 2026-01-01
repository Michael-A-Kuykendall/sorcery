// Test for $ prove: fn_is_callable -> test: callable_fn

import once from '../once';

function testCallableFn() {
    const fn = () => 'result';
    const wrapped = once(fn);

    if (typeof wrapped !== 'function') {
        throw new Error('Callable fn test failed: wrapped result should be callable');
    }

    const result = wrapped();

    if (result !== 'result') {
        throw new Error('Callable fn test failed: wrapped function should execute');
    }

    console.log('Callable fn test passed');
}

testCallableFn();