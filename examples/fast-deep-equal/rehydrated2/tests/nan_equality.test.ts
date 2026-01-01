// Test for $ prove: nan_values_are_equal -> test: nan_equality

import equal from '../equal';

function testNaNEquality() {
    // NaN values should be equal to each other
    const result1 = equal(NaN, NaN);
    if (result1 !== true) {
        throw new Error('NaN equality test failed: NaN should equal NaN');
    }

    // NaN should not equal other values
    const result2 = equal(NaN, 1);
    if (result2 !== false) {
        throw new Error('NaN equality test failed: NaN should not equal numbers');
    }

    const result3 = equal(NaN, 'NaN');
    if (result3 !== false) {
        throw new Error('NaN equality test failed: NaN should not equal strings');
    }

    console.log('NaN equality test passed');
}

testNaNEquality();