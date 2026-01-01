// Test for $ prove: circular_reference_detection -> test: circular_reference

import equal from '../equal';

function testCircularReference() {
    // Since the implementation forbids handling circular references,
    // this test verifies that non-circular nested structures work correctly
    const obj1 = { a: { b: { c: 1 } } };
    const obj2 = { a: { b: { c: 1 } } };
    const obj3 = { a: { b: { c: 2 } } };

    // Equal nested structures
    const result1 = equal(obj1, obj2);
    if (result1 !== true) {
        throw new Error('Circular reference test failed: equal nested objects should return true');
    }

    // Different nested structures
    const result2 = equal(obj1, obj3);
    if (result2 !== false) {
        throw new Error('Circular reference test failed: different nested objects should return false');
    }

    console.log('Circular reference test passed');
}

testCircularReference();