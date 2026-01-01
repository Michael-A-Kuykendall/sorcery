// Test for $ prove: exposes_called_flag_on_wrapper -> test: exposes_called

import once from '../once';

function testExposesCalled() {
    const fn = () => 'result';
    const wrapped = once(fn);

    if (typeof wrapped.called !== 'boolean') {
        throw new Error('Exposes called test failed: called property should exist');
    }

    if (wrapped.called !== false) {
        throw new Error('Exposes called test failed: called should initially be false');
    }

    wrapped();

    if (wrapped.called !== true) {
        throw new Error('Exposes called test failed: called should be true after execution');
    }

    console.log('Exposes called test passed');
}

testExposesCalled();