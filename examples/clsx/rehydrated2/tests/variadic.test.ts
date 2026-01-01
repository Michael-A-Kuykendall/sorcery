// Test for $ prove: accepts_variadic_arguments -> test: variadic

import clsx from '../clsx';

function testVariadic() {
    const result = clsx('foo', 'bar', 'baz');
    if (result !== 'foo bar baz') {
        throw new Error('Variadic test failed');
    }
    console.log('Variadic test passed');
}

testVariadic();