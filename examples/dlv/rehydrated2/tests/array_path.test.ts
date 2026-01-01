// Test for $ prove: accepts_array_path -> test: array_path

import dlv from '../dlv';

function testArrayPath() {
    const obj = { a: { b: { c: 'value' } } };
    const result = dlv(obj, ['a', 'b', 'c']);
    if (result !== 'value') {
        throw new Error('Array path test failed');
    }
    console.log('Array path test passed');
}

testArrayPath();