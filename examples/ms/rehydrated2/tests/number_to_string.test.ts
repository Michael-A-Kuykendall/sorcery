// Test for $ prove: number_input_returns_string -> test: number_to_string

import ms from '../ms';

function testNumberToString() {
    // Number input should return string
    const result1 = ms(1000);
    if (typeof result1 !== 'string') {
        throw new Error('Number to string test failed: number input should return string');
    }
    if (result1 !== '1s') {
        throw new Error('Number to string test failed: 1000 should return "1s"');
    }

    console.log('Number to string test passed');
}

testNumberToString();