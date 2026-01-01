// Test for $ prove: filters_falsy_values -> test: filters_falsy

import clsx from '../clsx';

function testFiltersFalsy() {
    const result = clsx('foo', null, undefined, false, '', 'bar');
    if (result !== 'foo bar') {
        throw new Error('Filters falsy test failed');
    }
    console.log('Filters falsy test passed');
}

testFiltersFalsy();