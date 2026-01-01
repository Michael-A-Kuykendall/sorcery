// Test for $ prove: day_24_hours -> test: day_24

import ms from '../ms';

function testDay24() {
    // Day should equal 24 hours
    const result1 = ms('1d');
    if (result1 !== 86400000) {
        throw new Error('Day 24 test failed: 1d should equal 86400000');
    }

    const result2 = ms('2d');
    if (result2 !== 172800000) {
        throw new Error('Day 24 test failed: 2d should equal 172800000');
    }

    console.log('Day 24 test passed');
}

testDay24();