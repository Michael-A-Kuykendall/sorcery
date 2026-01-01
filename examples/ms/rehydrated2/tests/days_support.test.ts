// Test for $ prove: supports_days_day_d -> test: days_support

import ms from '../ms';

function testDaysSupport() {
    const result1 = ms('1d');
    const result2 = ms('1day');
    const result3 = ms('1days');

    const expected = 24 * 60 * 60 * 1000; // 24 hours

    if (result1 !== expected) {
        throw new Error('Days support test failed: 1d should be 24 hours');
    }
    if (result2 !== result1) {
        throw new Error('Days support test failed: day should equal d');
    }
    if (result3 !== result1) {
        throw new Error('Days support test failed: days should equal d');
    }

    console.log('Days support test passed');
}

testDaysSupport();