// Test for $ prove: second_1000 -> test: second_1000

import ms from '../ms';

function testSecond1000() {
    // Second should equal 1000 milliseconds
    const result1 = ms('1s');
    if (result1 !== 1000) {
        throw new Error('Second 1000 test failed: 1s should equal 1000');
    }

    const result2 = ms('2s');
    if (result2 !== 2000) {
        throw new Error('Second 1000 test failed: 2s should equal 2000');
    }

    console.log('Second 1000 test passed');
}

testSecond1000();