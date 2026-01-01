// Test for $ prove: splits_string_on_dots -> test: splits_dots

import dlv from '../dlv';

function testSplitsDots() {
    const obj = { a: { b: { c: 'value' } } };
    const result = dlv(obj, 'a.b.c');
    if (result !== 'value') {
        throw new Error('Splits dots test failed');
    }
    console.log('Splits dots test passed');
}

testSplitsDots();