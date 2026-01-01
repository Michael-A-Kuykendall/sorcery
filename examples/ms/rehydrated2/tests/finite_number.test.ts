// Test for $ prove: ms_is_finite_number -> test: finite_number

import ms from '../ms';

function testFiniteNumber() {
    // Should accept finite numbers
    const result1 = ms(1000);
    if (typeof result1 !== 'string') {
        throw new Error('Finite number test failed: should accept 1000');
    }

    const result2 = ms(0);
    if (typeof result2 !== 'string') {
        throw new Error('Finite number test failed: should accept 0');
    }

    const result3 = ms(-1000);
    if (typeof result3 !== 'string') {
        throw new Error('Finite number test failed: should accept -1000');
    }

    console.log('Finite number test passed');
}

testFiniteNumber();