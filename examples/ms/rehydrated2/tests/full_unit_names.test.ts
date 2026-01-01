// Test for $ prove: uses_full_unit_names -> test: full_unit_names

import ms from '../ms';

function testFullUnitNames() {
    // Should use full unit names in long format
    const result1 = ms(1000, { long: true });
    if (result1 !== '1 second') {
        throw new Error('Full unit names test failed: 1000 with long should be "1 second"');
    }

    const result2 = ms(60000, { long: true });
    if (result2 !== '1 minute') {
        throw new Error('Full unit names test failed: 60000 with long should be "1 minute"');
    }

    const result3 = ms(3600000, { long: true });
    if (result3 !== '1 hour') {
        throw new Error('Full unit names test failed: 3600000 with long should be "1 hour"');
    }

    console.log('Full unit names test passed');
}

testFullUnitNames();