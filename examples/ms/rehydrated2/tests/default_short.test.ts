// Test for $ prove: default_is_short_format -> test: default_short

import ms from '../ms';

function testDefaultShort() {
    // Should default to short format
    const result1 = ms(1000);
    if (result1 !== '1s') {
        throw new Error('Default short test failed: 1000 should default to "1s"');
    }

    const result2 = ms(120000);
    if (result2 !== '2m') {
        throw new Error('Default short test failed: 120000 should default to "2m"');
    }

    console.log('Default short test passed');
}

testDefaultShort();