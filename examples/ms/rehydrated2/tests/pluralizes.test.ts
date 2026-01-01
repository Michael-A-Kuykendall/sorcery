// Test for $ prove: pluralizes_when_value_gte_1_5 -> test: pluralizes

import ms from '../ms';

function testPluralizes() {
    // Should pluralize when value >= 1.5
    const result1 = ms(2000, { long: true }); // 2 seconds
    if (result1 !== '2 seconds') {
        throw new Error('Pluralizes test failed: 2000 with long should be "2 seconds"');
    }

    const result2 = ms(120000, { long: true }); // 2 minutes
    if (result2 !== '2 minutes') {
        throw new Error('Pluralizes test failed: 120000 with long should be "2 minutes"');
    }

    // Should not pluralize when value < 1.5 (rounded to 1)
    const result3 = ms(1000, { long: true }); // 1 second
    if (result3 !== '1 second') {
        throw new Error('Pluralizes test failed: 1000 with long should be "1 second"');
    }

    console.log('Pluralizes test passed');
}

testPluralizes();