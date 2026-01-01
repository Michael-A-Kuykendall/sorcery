// Test for $ prove: both_must_be_objects_to_continue -> test: both_objects

import equal from '../equal';

function testBothObjects() {
    // Objects should continue comparison
    const result1 = equal({ a: 1 }, { a: 1 });
    if (result1 !== true) {
        throw new Error('Both objects test failed: equal objects should return true');
    }

    // One object, one primitive should return false
    const result2 = equal({ a: 1 }, 'string');
    if (result2 !== false) {
        throw new Error('Both objects test failed: object vs primitive should return false');
    }

    // Both primitives should use strict equality
    const result3 = equal(5, 5);
    if (result3 !== true) {
        throw new Error('Both objects test failed: equal primitives should return true');
    }

    console.log('Both objects test passed');
}

testBothObjects();