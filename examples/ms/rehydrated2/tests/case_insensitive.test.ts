// Test for $ prove: case_insensitive -> test: case_insensitive

import ms from '../ms';

function testCaseInsensitive() {
    // Should be case insensitive
    const result1 = ms('1S');
    const result2 = ms('1s');
    if (result1 !== result2) {
        throw new Error('Case insensitive test failed: 1S should equal 1s');
    }

    const result3 = ms('1HOUR');
    const result4 = ms('1hour');
    if (result3 !== result4) {
        throw new Error('Case insensitive test failed: 1HOUR should equal 1hour');
    }

    const result5 = ms('1Ms');
    const result6 = ms('1ms');
    if (result5 !== result6) {
        throw new Error('Case insensitive test failed: 1Ms should equal 1ms');
    }

    console.log('Case insensitive test passed');
}

testCaseInsensitive();