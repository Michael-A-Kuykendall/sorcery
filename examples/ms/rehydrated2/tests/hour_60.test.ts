// Test for $ prove: hour_60_minutes -> test: hour_60

import ms from '../ms';

function testHour60() {
    // Hour should equal 60 minutes
    const result1 = ms('1h');
    if (result1 !== 3600000) {
        throw new Error('Hour 60 test failed: 1h should equal 3600000');
    }

    const result2 = ms('2h');
    if (result2 !== 7200000) {
        throw new Error('Hour 60 test failed: 2h should equal 7200000');
    }

    console.log('Hour 60 test passed');
}

testHour60();