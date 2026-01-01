// Test for $ prove: first_call_executes_original -> test: first_call_executes

import once from '../once';

function testFirstCallExecutes() {
    let executed = false;
    const fn = () => { executed = true; return 'result'; };
    const wrapped = once(fn);

    wrapped();

    if (!executed) {
        throw new Error('First call executes test failed: original function should execute');
    }

    console.log('First call executes test passed');
}

testFirstCallExecutes();