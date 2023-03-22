export type Tests = {
    [test: string]: (() => unknown) | Tests;
}

export type Color = 'always' | 'never'
export type Format = 'human' | 'short' | 'json';

export interface Options {
    tests?: Array<string>,
    quiet?: boolean,
    verbose?: boolean,
    color?: Color,
    format?: Format
}

export interface Diagnostic {
    success: boolean,
    error: unknown
}

class Stdout {
    public constructor(private buffer: string) {}
}

function deep_clone<T>(value: T): T {
    if (typeof value !== 'object') {
        // non-reference types
        return value;
    }

    const result = Object.create(null);

    for (const key in value) {
        Object.defineProperty(result, key, {
            value: deep_clone(value[key])
        });
    }

    return result;
}

export function cargo_test(tests: Tests, options: Options = {color:'always',format:'human',quiet:false,verbose:false}) {
    const diagnostics: Array<Diagnostic> = [];

    function run_tests(tests: Tests) {
        for (const key in tests) {
            const value = tests[key];
    
            if (typeof value === 'object') {
                return run_tests(value);
            }
    
            try { } catch (e) {
                diagnostics.push({error:e})
            }       
        }
    
        return diagnostics;
    }
}

