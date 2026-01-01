// Test for $ prove: no_unit_defaults_to_milliseconds -> test: no_unit_ms

import ms from '../ms';

function testNoUnitMs() {
    // No unit should default to milliseconds
    const result1 = ms('1000');
    if (result1 !== 1000) {
        throw new Error('No unit ms test failed: "1000" should be 1000ms');
    }

    const result2 = ms('500');
    if (result2 !== 500) {
        throw new Error('No unit ms test failed: "500" should be 500ms');
    }

    console.log('No unit ms test passed');
}

testNoUnitMs();