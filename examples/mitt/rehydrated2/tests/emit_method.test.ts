// Test for $ prove: has_emit_method -> test: emit_method

import mitt from '../mitt';

function testEmitMethod() {
    const emitter = mitt();
    if (typeof emitter.emit !== 'function') {
        throw new Error('Emit method test failed: emitter should have emit method');
    }

    // Test that it can be called
    emitter.emit('test');
    console.log('Emit method test passed');
}

testEmitMethod();