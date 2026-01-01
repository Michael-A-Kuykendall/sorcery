// Test for $ prove: typed_arrays_equal_by_content -> test: typed_array_equality

import equal from '../equal';

function testTypedArrayEquality() {
    const arr1 = new Uint8Array([1, 2, 3]);
    const arr2 = new Uint8Array([1, 2, 3]);
    const arr3 = new Uint8Array([1, 2, 4]);
    const arr4 = new Uint8Array([1, 2]);

    // Equal typed arrays
    const result1 = equal(arr1, arr2);
    if (result1 !== true) {
        throw new Error('Typed array equality test failed: equal typed arrays should return true');
    }

    // Different values
    const result2 = equal(arr1, arr3);
    if (result2 !== false) {
        throw new Error('Typed array equality test failed: typed arrays with different values should return false');
    }

    // Different lengths
    const result3 = equal(arr1, arr4);
    if (result3 !== false) {
        throw new Error('Typed array equality test failed: typed arrays with different lengths should return false');
    }

    // Typed array vs regular array
    const result4 = equal(arr1, [1, 2, 3]);
    if (result4 !== false) {
        throw new Error('Typed array equality test failed: typed array vs regular array should return false');
    }

    console.log('Typed array equality test passed');
}

testTypedArrayEquality();