// Test for $ prove: supports_years_yrs_yr_y -> test: years_support

import ms from '../ms';

function testYearsSupport() {
    // Should support all year variations
    const result1 = ms('1y');
    const result2 = ms('1yr');
    const result3 = ms('1yrs');
    const result4 = ms('1year');
    const result5 = ms('1years');

    const expected = 365.25 * 24 * 60 * 60 * 1000; // Approximate year in ms

    if (Math.abs(result1 - expected) > 1000) {
        throw new Error('Years support test failed: 1y should be approximately one year');
    }
    if (result2 !== result1) {
        throw new Error('Years support test failed: yr should equal y');
    }
    if (result3 !== result1) {
        throw new Error('Years support test failed: yrs should equal y');
    }
    if (result4 !== result1) {
        throw new Error('Years support test failed: year should equal y');
    }
    if (result5 !== result1) {
        throw new Error('Years support test failed: years should equal y');
    }

    console.log('Years support test passed');
}

testYearsSupport();