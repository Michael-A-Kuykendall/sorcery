// Test for $ prove: throws_on_non_finite_number -> test: non_finite_throw

import ms from '../ms';

function testNonFiniteThrow() {
    // Should throw on non-finite numbers
    try {
        ms(Infinity);
        throw new Error('Should have thrown for Infinity');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Non-finite throw test failed: should throw TypeError for Infinity');
        }
    }

    try {
        ms(-Infinity);
        throw new Error('Should have thrown for -Infinity');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Non-finite throw test failed: should throw TypeError for -Infinity');
        }
    }

    try {
        ms(NaN);
        throw new Error('Should have thrown for NaN');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Non-finite throw test failed: should throw TypeError for NaN');
        }
    }

    console.log('Non-finite throw test passed');
}

testNonFiniteThrow();