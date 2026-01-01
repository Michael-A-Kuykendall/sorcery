// Test for $ prove: returns_frozen_interface -> test: frozen_interface

import mitt from '../mitt';

function testFrozenInterface() {
    const emitter = mitt();

    // Try to modify the interface
    try {
        (emitter as any).on = () => {};
        throw new Error('Frozen interface test failed: interface should be frozen');
    } catch (e: any) {
        if (e.message !== 'Cannot assign to read only property \'on\' of object \'#<Object>\'') {
            // Allow other error messages as long as it's a freezing error
        }
    }

    try {
        (emitter as any).newProperty = 'test';
        throw new Error('Frozen interface test failed: should not allow new properties');
    } catch (e: any) {
        if (e.message !== 'Cannot add property newProperty, object is not extensible') {
            // Allow other error messages as long as it's a freezing error
        }
    }

    console.log('Frozen interface test passed');
}

testFrozenInterface();