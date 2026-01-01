// Test for $ prove: minute_60_seconds -> test: minute_60

import ms from '../ms';

function testMinute60() {
    // Minute should equal 60 seconds
    const result1 = ms('1m');
    if (result1 !== 60000) {
        throw new Error('Minute 60 test failed: 1m should equal 60000');
    }

    const result2 = ms('2m');
    if (result2 !== 120000) {
        throw new Error('Minute 60 test failed: 2m should equal 120000');
    }

    console.log('Minute 60 test passed');
}

testMinute60();