// Test for $ prove: throws_on_empty_string -> test: empty_string_throw

import ms from '../ms';

function testEmptyStringThrow() {
    // Should throw on empty string
    try {
        ms('');
        throw new Error('Should have thrown for empty string');
    } catch (e) {
        if (!(e instanceof Error) || e.message !== 'Empty string') {
            throw new Error('Empty string throw test failed: should throw "Empty string"');
        }
    }

    try {
        ms('   ');
        throw new Error('Should have thrown for whitespace string');
    } catch (e) {
        if (!(e instanceof Error) || e.message !== 'Empty string') {
            throw new Error('Empty string throw test failed: should throw "Empty string" for whitespace');
        }
    }

    console.log('Empty string throw test passed');
}

testEmptyStringThrow();