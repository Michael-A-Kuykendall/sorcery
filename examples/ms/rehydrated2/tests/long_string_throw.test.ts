// Test for $ prove: throws_on_string_over_100_chars -> test: long_string_throw

import ms from '../ms';

function testLongStringThrow() {
    // Should throw on string over 100 characters
    const longString = 'a'.repeat(101);
    try {
        ms(longString);
        throw new Error('Should have thrown for long string');
    } catch (e) {
        if (!(e instanceof Error) || e.message !== 'String exceeds 100 characters') {
            throw new Error('Long string throw test failed: should throw "String exceeds 100 characters"');
        }
    }

    console.log('Long string throw test passed');
}

testLongStringThrow();