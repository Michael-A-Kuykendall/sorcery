// Test for $ prove: supports_seconds_secs_sec_s -> test: seconds_support

import ms from '../ms';

function testSecondsSupport() {
    const result1 = ms('1s');
    const result2 = ms('1sec');
    const result3 = ms('1secs');
    const result4 = ms('1second');
    const result5 = ms('1seconds');

    const expected = 1000; // 1000 milliseconds

    if (result1 !== expected) {
        throw new Error('Seconds support test failed: 1s should be 1000ms');
    }
    if (result2 !== result1) {
        throw new Error('Seconds support test failed: sec should equal s');
    }
    if (result3 !== result1) {
        throw new Error('Seconds support test failed: secs should equal s');
    }
    if (result4 !== result1) {
        throw new Error('Seconds support test failed: second should equal s');
    }
    if (result5 !== result1) {
        throw new Error('Seconds support test failed: seconds should equal s');
    }

    console.log('Seconds support test passed');
}

testSecondsSupport();