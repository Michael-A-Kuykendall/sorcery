// Test for $ prove: handles_null_in_chain -> test: null_chain

import dlv from '../dlv';

function testNullChain() {
    const obj = { a: { b: null } };
    const result = dlv(obj, 'a.b.c', 'default');
    if (result !== 'default') {
        throw new Error('Null chain test failed');
    }
    console.log('Null chain test passed');
}

testNullChain();