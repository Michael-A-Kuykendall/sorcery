// Test for $ prove: deep_object_comparison -> test: deep_object_equality

import equal from '../equal';

function testDeepObjectEquality() {
    // Deep equal objects
    const obj1 = { a: 1, b: { c: 2, d: [3, 4] } };
    const obj2 = { a: 1, b: { c: 2, d: [3, 4] } };

    const result1 = equal(obj1, obj2);
    if (result1 !== true) {
        throw new Error('Deep object equality test failed: equal nested objects should return true');
    }

    // Different nested objects
    const obj3 = { a: 1, b: { c: 2, d: [3, 5] } };
    const result2 = equal(obj1, obj3);
    if (result2 !== false) {
        throw new Error('Deep object equality test failed: different nested objects should return false');
    }

    // Different keys
    const obj4 = { a: 1, b: { c: 2, e: [3, 4] } };
    const result3 = equal(obj1, obj4);
    if (result3 !== false) {
        throw new Error('Deep object equality test failed: different keys should return false');
    }

    console.log('Deep object equality test passed');
}

testDeepObjectEquality();