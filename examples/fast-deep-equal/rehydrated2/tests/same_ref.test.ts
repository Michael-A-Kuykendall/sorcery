// Test for $ prove: same_reference_returns_true_immediately -> test: same_ref

import equal from '../equal';

function testSameRef() {
    const obj = { a: 1 };
    const result = equal(obj, obj);
    if (result !== true) {
        throw new Error('Same reference test failed: should return true for same object');
    }
    console.log('Same reference test passed');
}

testSameRef();