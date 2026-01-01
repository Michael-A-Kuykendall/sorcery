// Test for $ prove: supports_months_month_mo -> test: months_support

import ms from '../ms';

function testMonthsSupport() {
    // Should support all month variations
    const result1 = ms('1mo');
    const result2 = ms('1month');
    const result3 = ms('1months');

    const expected = (365.25 * 24 * 60 * 60 * 1000) / 12; // Year / 12

    if (Math.abs(result1 - expected) > 1000) {
        throw new Error('Months support test failed: 1mo should be approximately one month');
    }
    if (result2 !== result1) {
        throw new Error('Months support test failed: month should equal mo');
    }
    if (result3 !== result1) {
        throw new Error('Months support test failed: months should equal mo');
    }

    console.log('Months support test passed');
}

testMonthsSupport();