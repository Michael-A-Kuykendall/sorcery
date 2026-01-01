// Test for $ prove: supports_minutes_mins_min_m -> test: minutes_support

import ms from '../ms';

function testMinutesSupport() {
    const result1 = ms('1m');
    const result2 = ms('1min');
    const result3 = ms('1mins');
    const result4 = ms('1minute');
    const result5 = ms('1minutes');

    const expected = 60 * 1000; // 60 seconds

    if (result1 !== expected) {
        throw new Error('Minutes support test failed: 1m should be 60 seconds');
    }
    if (result2 !== result1) {
        throw new Error('Minutes support test failed: min should equal m');
    }
    if (result3 !== result1) {
        throw new Error('Minutes support test failed: mins should equal m');
    }
    if (result4 !== result1) {
        throw new Error('Minutes support test failed: minute should equal m');
    }
    if (result5 !== result1) {
        throw new Error('Minutes support test failed: minutes should equal m');
    }

    console.log('Minutes support test passed');
}

testMinutesSupport();