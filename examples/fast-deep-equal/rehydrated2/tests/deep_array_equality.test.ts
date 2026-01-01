// Test for $ prove: deep_array_comparison -> test: deep_array_equality

import equal from '../equal';

function testDeepArrayEquality() {
    // Deep equal arrays
    const arr1 = [1, [2, 3], { a: 4 }];
    const arr2 = [1, [2, 3], { a: 4 }];

    const result1 = equal(arr1, arr2);
    if (result1 !== true) {
        throw new Error('Deep array equality test failed: equal nested arrays should return true');
    }

    // Different nested arrays
    const arr3 = [1, [2, 4], { a: 4 }];
    const result2 = equal(arr1, arr3);
    if (result2 !== false) {
        throw new Error('Deep array equality test failed: different nested arrays should return false');
    }

    // Different lengths
    const arr4 = [1, [2, 3]];
    const result3 = equal(arr1, arr4);
    if (result3 !== false) {
        throw new Error('Deep array equality test failed: different length arrays should return false');
    }

    console.log('Deep array equality test passed');
}

testDeepArrayEquality();