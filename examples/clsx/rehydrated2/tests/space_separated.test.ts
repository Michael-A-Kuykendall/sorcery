// Test for $ prove: returns_space_separated_string -> test: space_separated

import clsx from '../clsx';

function testSpaceSeparated() {
    const result = clsx('foo', 'bar');
    if (result !== 'foo bar') {
        throw new Error('Space separated test failed');
    }
    console.log('Space separated test passed');
}

testSpaceSeparated();