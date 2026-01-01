// Test for $ prove: has_on_method -> test: on_method

import mitt from '../mitt';

function testOnMethod() {
    const emitter = mitt();
    if (typeof emitter.on !== 'function') {
        throw new Error('On method test failed: emitter should have on method');
    }

    // Test that it can be called
    emitter.on('test', () => {});
    console.log('On method test passed');
}

testOnMethod();