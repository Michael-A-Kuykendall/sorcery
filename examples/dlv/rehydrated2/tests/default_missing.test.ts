// Test for $ prove: returns_default_if_path_missing -> test: default_missing

import dlv from '../dlv';

function testDefaultMissing() {
    const obj = { a: { b: {} } };
    const result = dlv(obj, 'a.b.c', 'default');
    if (result !== 'default') {
        throw new Error('Default missing test failed');
    }
    console.log('Default missing test passed');
}

testDefaultMissing();