// Test for $ prove: accepts_string_path_with_dots -> test: string_path

import dlv from '../dlv';

function testStringPath() {
    const obj = { a: { b: { c: 'value' } } };
    const result = dlv(obj, 'a.b.c');
    if (result !== 'value') {
        throw new Error('String path test failed');
    }
    console.log('String path test passed');
}

testStringPath();