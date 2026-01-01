// Test for $ prove: allows_decimal_values -> test: decimal_values

import ms from '../ms';

function testDecimalValues() {
    // Should allow decimal values
    const result1 = ms('1.5s');
    if (result1 !== 1500) {
        throw new Error('Decimal values test failed: 1.5s should be 1500');
    }

    const result2 = ms('2.25m');
    if (result2 !== 135000) {
        throw new Error('Decimal values test failed: 2.25m should be 135000');
    }

    const result3 = ms('0.5h');
    if (result3 !== 1800000) {
        throw new Error('Decimal values test failed: 0.5h should be 1800000');
    }

    console.log('Decimal values test passed');
}

testDecimalValues();