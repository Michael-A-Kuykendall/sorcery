// Test for $ prove: called_flag_initially_false -> test: called_initial_false

import once from '../once';

function testCalledInitialFalse() {
    const fn = () => 'result';
    const wrapped = once(fn);

    if (wrapped.called !== false) {
        throw new Error('Called initial false test failed: called should start as false');
    }

    console.log('Called initial false test passed');
}

testCalledInitialFalse();