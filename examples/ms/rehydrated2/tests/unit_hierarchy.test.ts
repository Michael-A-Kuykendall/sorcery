// Test for $ prove: unit_hierarchy_y_mo_w_d_h_m_s_ms -> test: unit_hierarchy

import ms from '../ms';

function testUnitHierarchy() {
    // Should follow unit hierarchy: y > mo > w > d > h > m > s > ms
    const yearMs = 365.25 * 24 * 60 * 60 * 1000;
    const result1 = ms(yearMs);
    if (result1 !== '1y') {
        throw new Error('Unit hierarchy test failed: year should be "1y"');
    }

    const monthMs = (365.25 * 24 * 60 * 60 * 1000) / 12;
    const result2 = ms(monthMs);
    if (result2 !== '1mo') {
        throw new Error('Unit hierarchy test failed: month should be "1mo"');
    }

    const weekMs = 7 * 24 * 60 * 60 * 1000;
    const result3 = ms(weekMs);
    if (result3 !== '1w') {
        throw new Error('Unit hierarchy test failed: week should be "1w"');
    }

    const dayMs = 24 * 60 * 60 * 1000;
    const result4 = ms(dayMs);
    if (result4 !== '1d') {
        throw new Error('Unit hierarchy test failed: day should be "1d"');
    }

    const hourMs = 60 * 60 * 1000;
    const result5 = ms(hourMs);
    if (result5 !== '1h') {
        throw new Error('Unit hierarchy test failed: hour should be "1h"');
    }

    const minuteMs = 60 * 1000;
    const result6 = ms(minuteMs);
    if (result6 !== '1m') {
        throw new Error('Unit hierarchy test failed: minute should be "1m"');
    }

    const secondMs = 1000;
    const result7 = ms(secondMs);
    if (result7 !== '1s') {
        throw new Error('Unit hierarchy test failed: second should be "1s"');
    }

    const result8 = ms(500);
    if (result8 !== '500ms') {
        throw new Error('Unit hierarchy test failed: millisecond should be "500ms"');
    }

    console.log('Unit hierarchy test passed');
}

testUnitHierarchy();