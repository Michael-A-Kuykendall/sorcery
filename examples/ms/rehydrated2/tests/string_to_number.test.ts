// Test for $ prove: string_input_returns_number -> test: string_to_number

import ms from '../ms';

function testStringToNumber() {
    // String input should return number
    const result1 = ms('1s');
    if (typeof result1 !== 'number') {
        throw new Error('String to number test failed: string input should return number');
    }
    if (result1 !== 1000) {
        throw new Error('String to number test failed: 1s should equal 1000ms');
    }

    console.log('String to number test passed');
}

testStringToNumber();