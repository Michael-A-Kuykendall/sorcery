// Test for $ prove: uses_absolute_value_for_comparison -> test: absolute_value

import ms from '../ms';

function testAbsoluteValue() {
    // Should use absolute value for unit comparison but preserve sign
    const result1 = ms(-3600000); // -1 hour
    if (result1 !== '-1h') {
        throw new Error('Absolute value test failed: -3600000 should be "-1h"');
    }

    const result2 = ms(-120000); // -2 minutes
    if (result2 !== '-2m') {
        throw new Error('Absolute value test failed: -120000 should be "-2m"');
    }

    console.log('Absolute value test passed');
}

testAbsoluteValue();