// Test for $ prove: symbols_equal_by_reference -> test: symbol_equality

import equal from '../equal';

function testSymbolEquality() {
    const sym1 = Symbol('test');
    const sym2 = Symbol('test');
    const sym3 = sym1;

    // Different symbols should not be equal
    const result1 = equal(sym1, sym2);
    if (result1 !== false) {
        throw new Error('Symbol equality test failed: different symbols should not be equal');
    }

    // Same symbol reference should be equal
    const result2 = equal(sym1, sym3);
    if (result2 !== true) {
        throw new Error('Symbol equality test failed: same symbol reference should be equal');
    }

    // Symbol vs string
    const result3 = equal(sym1, 'test');
    if (result3 !== false) {
        throw new Error('Symbol equality test failed: symbol vs string should not be equal');
    }

    console.log('Symbol equality test passed');
}

testSymbolEquality();