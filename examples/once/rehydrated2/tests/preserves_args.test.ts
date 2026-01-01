// Test for $ prove: preserves_arguments_on_first_call -> test: preserves_args

import once from '../once';

function testPreservesArgs() {
    const receivedArgs: any[] = [];
    const fn = (...args: any[]) => { receivedArgs.push(...args); return 'result'; };
    const wrapped = once(fn);

    wrapped('arg1', 'arg2', 123);

    if (receivedArgs.length !== 3 || receivedArgs[0] !== 'arg1' || receivedArgs[1] !== 'arg2' || receivedArgs[2] !== 123) {
        throw new Error('Preserves args test failed: arguments should be preserved on first call');
    }

    console.log('Preserves args test passed');
}

testPreservesArgs();