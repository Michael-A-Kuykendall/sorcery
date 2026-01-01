// Test for $ prove: factory_function -> test: factory

import mitt from '../mitt';

function testFactory() {
    const emitter = mitt();
    if (typeof emitter !== 'object' || emitter === null) {
        throw new Error('Factory test failed: should return object');
    }
    if (typeof emitter.on !== 'function') {
        throw new Error('Factory test failed: should have on method');
    }
    if (typeof emitter.off !== 'function') {
        throw new Error('Factory test failed: should have off method');
    }
    if (typeof emitter.emit !== 'function') {
        throw new Error('Factory test failed: should have emit method');
    }
    console.log('Factory test passed');
}

testFactory();