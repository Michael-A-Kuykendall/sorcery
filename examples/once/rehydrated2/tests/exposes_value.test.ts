// Test for $ prove: exposes_value_property_on_wrapper -> test: exposes_value

import once from '../once';

function testExposesValue() {
    const fn = () => 'result';
    const wrapped = once(fn);

    if (!('value' in wrapped)) {
        throw new Error('Exposes value test failed: value property should exist');
    }

    if (wrapped.value !== undefined) {
        throw new Error('Exposes value test failed: value should initially be undefined');
    }

    wrapped();

    if (wrapped.value !== 'result') {
        throw new Error('Exposes value test failed: value should contain cached result');
    }

    console.log('Exposes value test passed');
}

testExposesValue();