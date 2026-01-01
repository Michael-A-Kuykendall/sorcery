// Test for $ prove: returns_NaN_for_unparseable -> test: nan_unparseable

import ms from '../ms';

function testNanUnparseable() {
    // Should return NaN for unparseable strings
    const result1 = ms('invalid');
    if (!isNaN(result1)) {
        throw new Error('NaN unparseable test failed: "invalid" should return NaN');
    }

    const result2 = ms('1xyz');
    if (!isNaN(result2)) {
        throw new Error('NaN unparseable test failed: "1xyz" should return NaN');
    }

    const result3 = ms('abc');
    if (!isNaN(result3)) {
        throw new Error('NaN unparseable test failed: "abc" should return NaN');
    }

    console.log('NaN unparseable test passed');
}

testNanUnparseable();