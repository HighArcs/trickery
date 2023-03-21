import { ChildProcess } from "child_process";
import { Writable } from "stream";
import { threadId, Worker } from "worker_threads";

export type Tests = {
    [test: string]: (() => unknown) | Tests;
}

export type Color = 'always' | 'never'
export type Format = 'human' | 'short' | 'json';

export interface Options {
    tests?: Array<string>,
    quiet?: boolean,
    tests_per_thread?: number,
    verbose?: boolean,
    color?: Color,
    format?: Format
}

export interface Diagnostic {
    success: boolean,
    message: string,
    errors: Array<string>
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

export function cargo_test(tests: Tests, options: Options = {color:'always',format:'human',quiet:false,verbose:false,tests_per_thread:1}) {
    const old_console = deep_clone(console);
    // adapt
    console = new console.Console(new Writable(), new Writable());

    const diagnostics: Array<Diagnostic> = [];

    const existing_thread = new ChildProcess({captureRejections:true}).on('message', (test) => {})
    let test_count = 0;

    function run_tests(tests: Tests) {
        for (const key in tests) {
            const value = tests[key];
    
            if (typeof value === 'object') {
                return run_tests(value);
            }
    
            if ()
            existing_thread.emit('message')
        }
    
        return diagnostics;
    }
    

    console = old_console;
}

