// Test for $ prove: supports_milliseconds_msecs_msec_ms -> test: milliseconds_support

import ms from '../ms';

function testMillisecondsSupport() {
    const result1 = ms('1ms');
    const result2 = ms('1msec');
    const result3 = ms('1msecs');
    const result4 = ms('1millisecond');
    const result5 = ms('1milliseconds');

    const expected = 1; // 1 millisecond

    if (result1 !== expected) {
        throw new Error('Milliseconds support test failed: 1ms should be 1');
    }
    if (result2 !== result1) {
        throw new Error('Milliseconds support test failed: msec should equal ms');
    }
    if (result3 !== result1) {
        throw new Error('Milliseconds support test failed: msecs should equal ms');
    }
    if (result4 !== result1) {
        throw new Error('Milliseconds support test failed: millisecond should equal ms');
    }
    if (result5 !== result1) {
        throw new Error('Milliseconds support test failed: milliseconds should equal ms');
    }

    console.log('Milliseconds support test passed');
}

testMillisecondsSupport();