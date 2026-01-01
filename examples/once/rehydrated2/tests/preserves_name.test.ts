// Test for $ prove: preserves_function_name -> test: preserves_name

import once from '../once';

function testPreservesName() {
    function namedFunction() { return 'result'; }
    const wrapped = once(namedFunction);

    if (wrapped.name !== 'namedFunction') {
        throw new Error('Preserves name test failed: function name should be preserved');
    }

    console.log('Preserves name test passed');
}

testPreservesName();