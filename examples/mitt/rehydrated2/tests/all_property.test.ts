// Test for $ prove: has_all_property -> test: all_property

import mitt from '../mitt';

function testAllProperty() {
    const emitter = mitt();
    if (!('all' in emitter)) {
        throw new Error('All property test failed: emitter should have all property');
    }
    if (emitter.all === undefined) {
        throw new Error('All property test failed: all property should not be undefined');
    }
    console.log('All property test passed');
}

testAllProperty();