// Test for $ prove: date_objects_equal_by_value -> test: date_equality

import equal from '../equal';

function testDateEquality() {
    const date1 = new Date('2023-01-01T00:00:00Z');
    const date2 = new Date('2023-01-01T00:00:00Z');
    const date3 = new Date('2023-01-02T00:00:00Z');

    // Same date values should be equal
    const result1 = equal(date1, date2);
    if (result1 !== true) {
        throw new Error('Date equality test failed: same dates should return true');
    }

    // Different date values should not be equal
    const result2 = equal(date1, date3);
    if (result2 !== false) {
        throw new Error('Date equality test failed: different dates should return false');
    }

    // Date vs non-date should not be equal
    const result3 = equal(date1, '2023-01-01');
    if (result3 !== false) {
        throw new Error('Date equality test failed: date vs string should return false');
    }

    console.log('Date equality test passed');
}

testDateEquality();