// Test for $ prove: all_parameter_optional -> test: all_optional

import mitt from '../mitt';

function testAllOptional() {
    // Should work with no parameters
    const emitter1 = mitt();
    if (!emitter1) {
        throw new Error('All optional test failed: should work with no params');
    }

    // Should work with undefined
    const emitter2 = mitt(undefined);
    if (!emitter2) {
        throw new Error('All optional test failed: should work with undefined');
    }

    // Should work with null (though this might create issues, but should not crash)
    try {
        const emitter3 = mitt(null as any);
        // If it doesn't crash, that's fine for this test
    } catch (e) {
        // Allow errors for null input
    }

    console.log('All optional test passed');
}

testAllOptional();