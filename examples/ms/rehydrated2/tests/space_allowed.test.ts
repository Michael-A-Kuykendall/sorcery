// Test for $ prove: allows_space_between_number_and_unit -> test: space_allowed

import ms from '../ms';

function testSpaceAllowed() {
    // Should allow space between number and unit
    const result1 = ms('1 s');
    if (result1 !== 1000) {
        throw new Error('Space allowed test failed: "1 s" should be 1000');
    }

    const result2 = ms('2 minutes');
    if (result2 !== 120000) {
        throw new Error('Space allowed test failed: "2 minutes" should be 120000');
    }

    const result3 = ms('1.5 hours');
    if (result3 !== 5400000) {
        throw new Error('Space allowed test failed: "1.5 hours" should be 5400000');
    }

    console.log('Space allowed test passed');
}

testSpaceAllowed();