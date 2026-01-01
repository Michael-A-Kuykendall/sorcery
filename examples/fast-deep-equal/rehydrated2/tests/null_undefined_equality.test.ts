// Test for $ prove: null_and_undefined_are_not_equal -> test: null_undefined_equality

import equal from '../equal';

function testNullUndefinedEquality() {
    // null and undefined should not be equal
    const result1 = equal(null, undefined);
    if (result1 !== false) {
        throw new Error('Null undefined equality test failed: null should not equal undefined');
    }

    // null equals null
    const result2 = equal(null, null);
    if (result2 !== true) {
        throw new Error('Null undefined equality test failed: null should equal null');
    }

    // undefined equals undefined
    const result3 = equal(undefined, undefined);
    if (result3 !== true) {
        throw new Error('Null undefined equality test failed: undefined should equal undefined');
    }

    console.log('Null undefined equality test passed');
}

testNullUndefinedEquality();