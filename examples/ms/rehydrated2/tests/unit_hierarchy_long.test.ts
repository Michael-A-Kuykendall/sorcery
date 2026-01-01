// Test for $ prove: unit_hierarchy_y_mo_w_d_h_m_s_ms -> test: unit_hierarchy_long

import ms from '../ms';

function testUnitHierarchyLong() {
    // Should follow unit hierarchy in long format: y > mo > w > d > h > m > s > ms
    const yearMs = 365.25 * 24 * 60 * 60 * 1000;
    const result1 = ms(yearMs, { long: true });
    if (result1 !== '1 year') {
        throw new Error('Unit hierarchy long test failed: year should be "1 year"');
    }

    const monthMs = (365.25 * 24 * 60 * 60 * 1000) / 12;
    const result2 = ms(monthMs, { long: true });
    if (result2 !== '1 month') {
        throw new Error('Unit hierarchy long test failed: month should be "1 month"');
    }

    const weekMs = 7 * 24 * 60 * 60 * 1000;
    const result3 = ms(weekMs, { long: true });
    if (result3 !== '1 week') {
        throw new Error('Unit hierarchy long test failed: week should be "1 week"');
    }

    const dayMs = 24 * 60 * 60 * 1000;
    const result4 = ms(dayMs, { long: true });
    if (result4 !== '1 day') {
        throw new Error('Unit hierarchy long test failed: day should be "1 day"');
    }

    const hourMs = 60 * 60 * 1000;
    const result5 = ms(hourMs, { long: true });
    if (result5 !== '1 hour') {
        throw new Error('Unit hierarchy long test failed: hour should be "1 hour"');
    }

    const minuteMs = 60 * 1000;
    const result6 = ms(minuteMs, { long: true });
    if (result6 !== '1 minute') {
        throw new Error('Unit hierarchy long test failed: minute should be "1 minute"');
    }

    const secondMs = 1000;
    const result7 = ms(secondMs, { long: true });
    if (result7 !== '1 second') {
        throw new Error('Unit hierarchy long test failed: second should be "1 second"');
    }

    const result8 = ms(500, { long: true });
    if (result8 !== '500 ms') {
        throw new Error('Unit hierarchy long test failed: millisecond should be "500 ms"');
    }

    console.log('Unit hierarchy long test passed');
}

testUnitHierarchyLong();