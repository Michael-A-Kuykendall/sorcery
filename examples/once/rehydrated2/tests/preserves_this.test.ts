// Test for $ prove: preserves_this_context -> test: preserves_this

import once from '../once';

function testPreservesThis() {
    const context = { value: 'test' };
    const fn = function(this: any) { return this.value; };
    const wrapped = once(fn);

    const result = wrapped.call(context);

    if (result !== 'test') {
        throw new Error('Preserves this test failed: this context should be preserved');
    }

    console.log('Preserves this test passed');
}

testPreservesThis();