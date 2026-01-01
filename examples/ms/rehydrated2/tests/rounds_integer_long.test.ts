// Test for $ prove: rounds_to_integer -> test: rounds_integer_long

import ms from '../ms';

function testRoundsIntegerLong() {
    // Should round to integer in long format
    const result1 = ms(1500, { long: true }); // 1.5 seconds
    if (result1 !== '2 seconds') {
        throw new Error('Rounds integer long test failed: 1500 with long should round to "2 seconds"');
    }

    const result2 = ms(90000, { long: true }); // 1.5 minutes
    if (result2 !== '2 minutes') {
        throw new Error('Rounds integer long test failed: 90000 with long should round to "2 minutes"');
    }

    console.log('Rounds integer long test passed');
}

testRoundsIntegerLong();