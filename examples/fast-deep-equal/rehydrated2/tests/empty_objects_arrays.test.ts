// Test for $ prove: empty_objects_and_arrays_are_equal -> test: empty_objects_arrays

import equal from '../equal';

function testEmptyObjectsArrays() {
    // Empty objects are equal
    const result1 = equal({}, {});
    if (result1 !== true) {
        throw new Error('Empty objects arrays test failed: empty objects should be equal');
    }

    // Empty arrays are equal
    const result2 = equal([], []);
    if (result2 !== true) {
        throw new Error('Empty objects arrays test failed: empty arrays should be equal');
    }

    // Empty object vs empty array should not be equal
    const result3 = equal({}, []);
    if (result3 !== false) {
        throw new Error('Empty objects arrays test failed: empty object vs empty array should not be equal');
    }

    console.log('Empty objects arrays test passed');
}

testEmptyObjectsArrays();