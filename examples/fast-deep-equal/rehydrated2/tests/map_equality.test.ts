// Test for $ prove: map_objects_equal_by_content -> test: map_equality

import equal from '../equal';

function testMapEquality() {
    const map1 = new Map([['a', 1], ['b', 2]]);
    const map2 = new Map([['a', 1], ['b', 2]]);
    const map3 = new Map([['a', 1], ['b', 3]]);
    const map4 = new Map([['a', 1]]);

    // Equal maps
    const result1 = equal(map1, map2);
    if (result1 !== true) {
        throw new Error('Map equality test failed: equal maps should return true');
    }

    // Different values
    const result2 = equal(map1, map3);
    if (result2 !== false) {
        throw new Error('Map equality test failed: maps with different values should return false');
    }

    // Different sizes
    const result3 = equal(map1, map4);
    if (result3 !== false) {
        throw new Error('Map equality test failed: maps with different sizes should return false');
    }

    // Maps with object keys (reference equality)
    const key1 = { k: 1 };
    const key2 = { k: 1 };
    const map5 = new Map([[key1, 'value']]);
    const map6 = new Map([[key1, 'value']]); // Same key reference
    const map7 = new Map([[key2, 'value']]); // Different key reference

    const result4 = equal(map5, map6);
    if (result4 !== true) {
        throw new Error('Map equality test failed: maps with same key references should return true');
    }

    const result5 = equal(map5, map7);
    if (result5 !== false) {
        throw new Error('Map equality test failed: maps with different key references should return false');
    }

    console.log('Map equality test passed');
}

testMapEquality();