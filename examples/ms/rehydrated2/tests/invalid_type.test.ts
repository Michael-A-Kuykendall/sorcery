// Test for $ prove: throws_on_invalid_type -> test: invalid_type

import ms from '../ms';

function testInvalidType() {
    // Invalid types should throw
    try {
        ms(null as any);
        throw new Error('Should have thrown for null');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Invalid type test failed: should throw TypeError for null');
        }
    }

    try {
        ms(undefined as any);
        throw new Error('Should have thrown for undefined');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Invalid type test failed: should throw TypeError for undefined');
        }
    }

    try {
        ms({} as any);
        throw new Error('Should have thrown for object');
    } catch (e) {
        if (!(e instanceof TypeError)) {
            throw new Error('Invalid type test failed: should throw TypeError for object');
        }
    }

    console.log('Invalid type test passed');
}

testInvalidType();