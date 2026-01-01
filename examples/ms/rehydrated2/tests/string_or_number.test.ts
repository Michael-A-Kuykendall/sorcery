// Test for $ prove: value_is_string_or_number -> test: string_or_number

import ms from '../ms';

function testStringOrNumber() {
    // Should accept strings
    const result1 = ms('1s');
    if (typeof result1 !== 'number') {
        throw new Error('String or number test failed: should accept strings');
    }

    // Should accept numbers
    const result2 = ms(1000);
    if (typeof result2 !== 'string') {
        throw new Error('String or number test failed: should accept numbers');
    }

    console.log('String or number test passed');
}

testStringOrNumber();