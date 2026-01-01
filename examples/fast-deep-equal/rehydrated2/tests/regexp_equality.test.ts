// Test for $ prove: regexp_objects_equal_by_value -> test: regexp_equality

import equal from '../equal';

function testRegexpEquality() {
    const regex1 = /test/gi;
    const regex2 = /test/gi;
    const regex3 = /test/g;
    const regex4 = /other/gi;

    // Same regex patterns and flags
    const result1 = equal(regex1, regex2);
    if (result1 !== true) {
        throw new Error('Regexp equality test failed: same regex should return true');
    }

    // Different flags
    const result2 = equal(regex1, regex3);
    if (result2 !== false) {
        throw new Error('Regexp equality test failed: different flags should return false');
    }

    // Different patterns
    const result3 = equal(regex1, regex4);
    if (result3 !== false) {
        throw new Error('Regexp equality test failed: different patterns should return false');
    }

    // Regex vs string
    const result4 = equal(regex1, '/test/gi');
    if (result4 !== false) {
        throw new Error('Regexp equality test failed: regex vs string should return false');
    }

    console.log('Regexp equality test passed');
}

testRegexpEquality();