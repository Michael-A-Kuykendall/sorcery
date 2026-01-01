// Test for $ prove: map_injected_or_created -> test: map_inject

import mitt from '../mitt';

function testMapInject() {
    // Test 1: No map provided, should create new map
    const emitter1 = mitt();
    if (!emitter1.all) {
        throw new Error('Map inject test failed: should create map when none provided');
    }
    if (emitter1.all.size !== 0) {
        throw new Error('Map inject test failed: new map should be empty');
    }

    // Test 2: Map provided, should use it
    const existingMap = new Map();
    existingMap.set('preexisting', [() => {}]);
    const emitter2 = mitt(existingMap);
    if (emitter2.all !== existingMap) {
        throw new Error('Map inject test failed: should use provided map');
    }
    if (!emitter2.all.has('preexisting')) {
        throw new Error('Map inject test failed: should preserve existing map contents');
    }

    console.log('Map inject test passed');
}

testMapInject();