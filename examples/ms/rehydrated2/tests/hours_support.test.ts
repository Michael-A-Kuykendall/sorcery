// Test for $ prove: supports_hours_hrs_hr_h -> test: hours_support

import ms from '../ms';

function testHoursSupport() {
    const result1 = ms('1h');
    const result2 = ms('1hr');
    const result3 = ms('1hrs');
    const result4 = ms('1hour');
    const result5 = ms('1hours');

    const expected = 60 * 60 * 1000; // 60 minutes

    if (result1 !== expected) {
        throw new Error('Hours support test failed: 1h should be 60 minutes');
    }
    if (result2 !== result1) {
        throw new Error('Hours support test failed: hr should equal h');
    }
    if (result3 !== result1) {
        throw new Error('Hours support test failed: hrs should equal h');
    }
    if (result4 !== result1) {
        throw new Error('Hours support test failed: hour should equal h');
    }
    if (result5 !== result1) {
        throw new Error('Hours support test failed: hours should equal h');
    }

    console.log('Hours support test passed');
}

testHoursSupport();