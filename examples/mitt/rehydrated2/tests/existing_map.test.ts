// Test for $ prove: accepts_existing_map -> test: existing_map

import mitt, { HandlerMap } from '../mitt';

function testExistingMap() {
    const existingMap = new Map();
    existingMap.set('test', [() => {}]);

    const emitter = mitt(existingMap);

    if (emitter.all !== existingMap) {
        throw new Error('Existing map test failed: should use provided map');
    }

    // Test that existing handlers work
    let called = false;
    emitter.on('existing', () => { called = true; });
    emitter.emit('existing');

    if (!called) {
        throw new Error('Existing map test failed: existing map should be functional');
    }

    console.log('Existing map test passed');
}

testExistingMap();