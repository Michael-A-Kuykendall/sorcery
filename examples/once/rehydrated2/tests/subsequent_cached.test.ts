// Test for $ prove: subsequent_calls_return_cached_value -> test: subsequent_cached

import once from '../once';

function testSubsequentCached() {
    let callCount = 0;
    const fn = () => { callCount++; return 'result'; };
    const wrapped = once(fn);

    const result1 = wrapped();
    const result2 = wrapped();
    const result3 = wrapped();

    if (callCount !== 1) {
        throw new Error('Subsequent cached test failed: original function should only execute once');
    }

    if (result1 !== 'result' || result2 !== 'result' || result3 !== 'result') {
        throw new Error('Subsequent cached test failed: all calls should return cached value');
    }

    console.log('Subsequent cached test passed');
}

testSubsequentCached();