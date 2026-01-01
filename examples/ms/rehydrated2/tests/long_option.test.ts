// Test for $ prove: options_long_selects_verbose_format -> test: long_option

import ms from '../ms';

function testLongOption() {
    // Should use long format when options.long is true
    const result1 = ms(1000, { long: true });
    if (result1 !== '1 second') {
        throw new Error('Long option test failed: 1000 with long should be "1 second"');
    }

    const result2 = ms(120000, { long: true });
    if (result2 !== '2 minutes') {
        throw new Error('Long option test failed: 120000 with long should be "2 minutes"');
    }

    console.log('Long option test passed');
}

testLongOption();