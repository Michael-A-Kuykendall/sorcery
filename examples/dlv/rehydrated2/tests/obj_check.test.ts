// Test for $ prove: obj_is_object_or_nullish -> test: obj_check

import dlv from '../dlv';

function testObjCheck() {
    // Should handle null
    const result1 = dlv(null, 'a.b.c', 'default');
    if (result1 !== 'default') {
        throw new Error('Obj check test failed for null');
    }

    // Should handle undefined
    const result2 = dlv(undefined, 'a.b.c', 'default');
    if (result2 !== 'default') {
        throw new Error('Obj check test failed for undefined');
    }

    // Should handle valid object
    const result3 = dlv({ a: 'value' }, 'a');
    if (result3 !== 'value') {
        throw new Error('Obj check test failed for valid object');
    }

    console.log('Obj check test passed');
}

testObjCheck();