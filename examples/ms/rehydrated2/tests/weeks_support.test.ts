// Test for $ prove: supports_weeks_week_w -> test: weeks_support

import ms from '../ms';

function testWeeksSupport() {
    const result1 = ms('1w');
    const result2 = ms('1week');
    const result3 = ms('1weeks');

    const expected = 7 * 24 * 60 * 60 * 1000; // 7 days

    if (result1 !== expected) {
        throw new Error('Weeks support test failed: 1w should be 7 days');
    }
    if (result2 !== result1) {
        throw new Error('Weeks support test failed: week should equal w');
    }
    if (result3 !== result1) {
        throw new Error('Weeks support test failed: weeks should equal w');
    }

    console.log('Weeks support test passed');
}

testWeeksSupport();