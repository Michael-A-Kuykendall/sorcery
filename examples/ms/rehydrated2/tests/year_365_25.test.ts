// Test for $ prove: year_365_25_days -> test: year_365_25

import ms from '../ms';

function testYear36525() {
    // Year should equal 365.25 days
    const expected = 365.25 * 24 * 60 * 60 * 1000;
    const result1 = ms('1y');
    if (Math.abs(result1 - expected) > 1000) {
        throw new Error('Year 365.25 test failed: 1y should equal approximately 31557600000');
    }

    const result2 = ms('2y');
    if (Math.abs(result2 - (expected * 2)) > 1000) {
        throw new Error('Year 365.25 test failed: 2y should equal approximately 63115200000');
    }

    console.log('Year 365.25 test passed');
}

testYear36525();