// Test for $ prove: returns_undefined_if_no_default_and_missing -> test: undefined_missing

import dlv from '../dlv';

function testUndefinedMissing() {
    const obj = { a: { b: {} } };
    const result = dlv(obj, 'a.b.c');
    if (result !== undefined) {
        throw new Error('Undefined missing test failed');
    }
    console.log('Undefined missing test passed');
}

testUndefinedMissing();