// Test for $ prove: handles_undefined_in_chain -> test: undefined_chain

import dlv from '../dlv';

function testUndefinedChain() {
    const obj = { a: { b: undefined } };
    const result = dlv(obj, 'a.b.c', 'default');
    if (result !== 'default') {
        throw new Error('Undefined chain test failed');
    }
    console.log('Undefined chain test passed');
}

testUndefinedChain();