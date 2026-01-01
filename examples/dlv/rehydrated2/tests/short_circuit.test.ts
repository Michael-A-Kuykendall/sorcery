// Test for $ prove: short_circuits_on_nullish -> test: short_circuit

import dlv from '../dlv';

function testShortCircuit() {
    const obj = { a: { b: null, c: { d: 'should_not_reach' } } };
    const result = dlv(obj, 'a.b.c.d', 'default');
    if (result !== 'default') {
        throw new Error('Short circuit test failed');
    }
    console.log('Short circuit test passed');
}

testShortCircuit();