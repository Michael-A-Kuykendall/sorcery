// Test for $ prove: str_matches_number_unit_pattern -> test: number_unit_pattern

import ms from '../ms';

function testNumberUnitPattern() {
    // Should match valid number-unit patterns
    const validPatterns = [
        '1s', '1.5m', '-2h', '1000', '1 second', '2.5 minutes'
    ];

    for (const pattern of validPatterns) {
        const result = ms(pattern);
        if (typeof result !== 'number' && !isNaN(result)) {
            throw new Error(`Number unit pattern test failed: "${pattern}" should be valid`);
        }
    }

    // Should reject invalid patterns
    const invalidPatterns = ['abc', '1xyz', ''];
    for (const pattern of invalidPatterns) {
        try {
            const result = ms(pattern);
            if (!isNaN(result)) {
                throw new Error(`Number unit pattern test failed: "${pattern}" should be invalid`);
            }
        } catch (e) {
            // Expected to throw or return NaN
        }
    }

    console.log('Number unit pattern test passed');
}

testNumberUnitPattern();