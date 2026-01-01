// Test for $ prove: month_year_div_12 -> test: month_year_div

import ms from '../ms';

function testMonthYearDiv() {
    // Month should equal year divided by 12
    const yearMs = 365.25 * 24 * 60 * 60 * 1000;
    const monthMs = yearMs / 12;
    const result1 = ms('1mo');
    if (Math.abs(result1 - monthMs) > 1000) {
        throw new Error('Month year div test failed: 1mo should equal year/12');
    }

    const result2 = ms('12mo');
    if (Math.abs(result2 - yearMs) > 1000) {
        throw new Error('Month year div test failed: 12mo should equal 1 year');
    }

    console.log('Month year div test passed');
}

testMonthYearDiv();