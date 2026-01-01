// Test for $ prove: functions_are_never_equal -> test: function_equality

import equal from '../equal';

function testFunctionEquality() {
    const func1 = () => 1;
    const func2 = () => 1;
    const func3 = func1;

    // Different functions should not be equal
    const result1 = equal(func1, func2);
    if (result1 !== false) {
        throw new Error('Function equality test failed: different functions should not be equal');
    }

    // Same function reference should be equal (reference equality takes precedence)
    const result2 = equal(func1, func3);
    if (result2 !== true) {
        throw new Error('Function equality test failed: same function reference should be equal');
    }

    // Function vs non-function should not be equal
    const result3 = equal(func1, {});
    if (result3 !== false) {
        throw new Error('Function equality test failed: function vs object should not be equal');
    }

    console.log('Function equality test passed');
}

testFunctionEquality();