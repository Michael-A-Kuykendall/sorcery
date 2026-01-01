// Test for $ prove: has_off_method -> test: off_method

import mitt from '../mitt';

function testOffMethod() {
    const emitter = mitt();
    if (typeof emitter.off !== 'function') {
        throw new Error('Off method test failed: emitter should have off method');
    }

    // Test that it can be called
    emitter.off('test');
    console.log('Off method test passed');
}

testOffMethod();