// Test for $ prove: regex_based_parsing -> test: regex_parsing

import ms from '../ms';

function testRegexParsing() {
    // Should parse various formats using regex
    const result1 = ms('1s');
    if (result1 !== 1000) {
        throw new Error('Regex parsing test failed: 1s should be 1000');
    }

    const result2 = ms('2 minutes');
    if (result2 !== 120000) {
        throw new Error('Regex parsing test failed: 2 minutes should be 120000');
    }

    const result3 = ms('1.5h');
    if (result3 !== 5400000) {
        throw new Error('Regex parsing test failed: 1.5h should be 5400000');
    }

    console.log('Regex parsing test passed');
}

testRegexParsing();