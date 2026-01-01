// Test for $ prove: uses_abbreviated_units -> test: abbreviated_units

import ms from '../ms';

function testAbbreviatedUnits() {
    // Should use abbreviated units
    const result1 = ms(1000);
    if (result1 !== '1s') {
        throw new Error('Abbreviated units test failed: 1000 should be "1s"');
    }

    const result2 = ms(60000);
    if (result2 !== '1m') {
        throw new Error('Abbreviated units test failed: 60000 should be "1m"');
    }

    const result3 = ms(3600000);
    if (result3 !== '1h') {
        throw new Error('Abbreviated units test failed: 3600000 should be "1h"');
    }

    console.log('Abbreviated units test passed');
}

testAbbreviatedUnits();