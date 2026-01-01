// Test for $ prove: rounds_to_integer -> test: rounds_integer

import ms from '../ms';

function testRoundsInteger() {
    // Should round to integer
    const result1 = ms(1500); // 1.5 seconds
    if (result1 !== '2s') {
        throw new Error('Rounds integer test failed: 1500 should round to "2s"');
    }

    const result2 = ms(90000); // 1.5 minutes
    if (result2 !== '2m') {
        throw new Error('Rounds integer test failed: 90000 should round to "2m"');
    }

    console.log('Rounds integer test passed');
}

testRoundsInteger();