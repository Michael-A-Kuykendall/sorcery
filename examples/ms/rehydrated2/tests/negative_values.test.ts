// Test for $ prove: allows_negative_values -> test: negative_values

import ms from '../ms';

function testNegativeValues() {
    // Should allow negative values
    const result1 = ms('-1s');
    if (result1 !== -1000) {
        throw new Error('Negative values test failed: -1s should be -1000');
    }

    const result2 = ms('-2.5m');
    if (result2 !== -150000) {
        throw new Error('Negative values test failed: -2.5m should be -150000');
    }

    console.log('Negative values test passed');
}

testNegativeValues();