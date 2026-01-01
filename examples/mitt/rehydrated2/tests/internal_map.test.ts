// Test for $ prove: exposes_internal_map -> test: internal_map

import mitt, { HandlerMap } from '../mitt';

function testInternalMap() {
    const emitter = mitt();
    if (!(emitter.all instanceof Map)) {
        throw new Error('Internal map test failed: all should be a Map');
    }

    // Test that we can access the internal map
    const map = emitter.all as HandlerMap;
    if (typeof map.get !== 'function') {
        throw new Error('Internal map test failed: should have Map methods');
    }

    console.log('Internal map test passed');
}

testInternalMap();