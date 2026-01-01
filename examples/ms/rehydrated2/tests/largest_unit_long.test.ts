// Test for $ prove: returns_largest_fitting_unit -> test: largest_unit_long

import ms from '../ms';

function testLargestUnitLong() {
    // Should return largest fitting unit in long format
    const result1 = ms(3661000, { long: true }); // Just over 1 hour
    if (result1 !== '1 hour') {
        throw new Error('Largest unit long test failed: 3661000 with long should be "1 hour"');
    }

    const result2 = ms(90000, { long: true }); // 1.5 minutes
    if (result2 !== '2 minutes') {
        throw new Error('Largest unit long test failed: 90000 with long should be "2 minutes"');
    }

    console.log('Largest unit long test passed');
}

testLargestUnitLong();