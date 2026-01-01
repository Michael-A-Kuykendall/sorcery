// Test for $ prove: returns_actual_value_if_found -> test: actual_value

import dlv from '../dlv';

function testActualValue() {
    const obj = { a: { b: { c: 'found' } } };
    const result = dlv(obj, 'a.b.c');
    if (result !== 'found') {
        throw new Error('Actual value test failed');
    }
    console.log('Actual value test passed');
}

testActualValue();