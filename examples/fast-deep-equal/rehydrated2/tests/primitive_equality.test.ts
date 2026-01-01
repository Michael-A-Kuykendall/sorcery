// Test for $ prove: primitive_values_use_strict_equality -> test: primitive_equality

import equal from '../equal';

function testPrimitiveEquality() {
    // Equal primitives
    const result1 = equal(5, 5);
    if (result1 !== true) {
        throw new Error('Primitive equality test failed: equal numbers should return true');
    }

    const result2 = equal('hello', 'hello');
    if (result2 !== true) {
        throw new Error('Primitive equality test failed: equal strings should return true');
    }

    const result3 = equal(true, true);
    if (result3 !== true) {
        throw new Error('Primitive equality test failed: equal booleans should return true');
    }

    // Different primitives
    const result4 = equal(5, 6);
    if (result4 !== false) {
        throw new Error('Primitive equality test failed: different numbers should return false');
    }

    const result5 = equal('hello', 'world');
    if (result5 !== false) {
        throw new Error('Primitive equality test failed: different strings should return false');
    }

    const result6 = equal(true, false);
    if (result6 !== false) {
        throw new Error('Primitive equality test failed: different booleans should return false');
    }

    console.log('Primitive equality test passed');
}

testPrimitiveEquality();