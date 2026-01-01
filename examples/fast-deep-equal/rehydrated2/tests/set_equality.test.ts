// Test for $ prove: set_objects_equal_by_content -> test: set_equality

import equal from '../equal';

function testSetEquality() {
    const set1 = new Set([1, 2, 3]);
    const set2 = new Set([1, 2, 3]);
    const set3 = new Set([1, 2, 4]);
    const set4 = new Set([1, 2]);

    // Equal sets with primitives
    const result1 = equal(set1, set2);
    if (result1 !== true) {
        throw new Error('Set equality test failed: equal sets should return true');
    }

    // Different values
    const result2 = equal(set1, set3);
    if (result2 !== false) {
        throw new Error('Set equality test failed: sets with different values should return false');
    }

    // Different sizes
    const result3 = equal(set1, set4);
    if (result3 !== false) {
        throw new Error('Set equality test failed: sets with different sizes should return false');
    }

    // Sets with objects (reference equality)
    const obj1 = { a: 1 };
    const obj2 = { a: 1 };
    const set5 = new Set([obj1]);
    const set6 = new Set([obj1]); // Same reference
    const set7 = new Set([obj2]); // Different reference, same content

    const result4 = equal(set5, set6);
    if (result4 !== true) {
        throw new Error('Set equality test failed: sets with same object references should return true');
    }

    const result5 = equal(set5, set7);
    if (result5 !== false) {
        throw new Error('Set equality test failed: sets with different object references should return false');
    }

    console.log('Set equality test passed');
}

testSetEquality();