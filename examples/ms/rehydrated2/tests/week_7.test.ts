// Test for $ prove: week_7_days -> test: week_7

import ms from '../ms';

function testWeek7() {
    // Week should equal 7 days
    const result1 = ms('1w');
    if (result1 !== 604800000) {
        throw new Error('Week 7 test failed: 1w should equal 604800000');
    }

    const result2 = ms('2w');
    if (result2 !== 1209600000) {
        throw new Error('Week 7 test failed: 2w should equal 1209600000');
    }

    console.log('Week 7 test passed');
}

testWeek7();