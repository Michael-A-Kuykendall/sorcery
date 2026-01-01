// Test for $ prove: returns_largest_fitting_unit -> test: largest_unit

import ms from '../ms';

function testLargestUnit() {
    // Should return largest fitting unit
    const result1 = ms(3661000); // Just over 1 hour
    if (result1 !== '1h') {
        throw new Error('Largest unit test failed: 3661000 should be "1h"');
    }

    const result2 = ms(90000); // 1.5 minutes
    if (result2 !== '2m') {
        throw new Error('Largest unit test failed: 90000 should be "2m"');
    }

    const result3 = ms(500); // 500ms
    if (result3 !== '500ms') {
        throw new Error('Largest unit test failed: 500 should be "500ms"');
    }

    console.log('Largest unit test passed');
}

testLargestUnit();