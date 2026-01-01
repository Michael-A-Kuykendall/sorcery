// Test for $ prove: preserves_function_length -> test: preserves_length

import once from '../once';

function testPreservesLength() {
    const fn = (a: any, b: any, c: any) => 'result';
    const wrapped = once(fn);

    if (wrapped.length !== 3) {
        throw new Error('Preserves length test failed: function length should be preserved');
    }

    console.log('Preserves length test passed');
}

testPreservesLength();