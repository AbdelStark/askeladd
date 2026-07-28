/* tslint:disable */
/* eslint-disable */

/**
 * Result type handed back to JavaScript: a success flag plus a message
 * (the serialized proof on success, the error otherwise).
 */
export class StwoResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly message: string;
    readonly success: boolean;
}

export function prove_and_verify(log_n_instances: number): StwoResult;

export function prove_and_verify_fib(log_size: number, claim: number): StwoResult;

export function prove_stark_proof_poseidon(log_n_instances: number): StwoResult;

export function stark_proof_multi_fibo(log_sizes: Uint32Array, claims_int: Uint32Array): StwoResult;

export function stark_proof_wide_fibo(log_fibonacci_size: number, log_n_instances: number): StwoResult;

export function verify_stark_proof(log_n_instances: number, stark_proof_str: string): StwoResult;

export function verify_stark_proof_fib(log_size: number, claim: number, stark_proof_str: string): StwoResult;

export function verify_stark_proof_multi_fibo(log_sizes: Uint32Array, claims_int: Uint32Array, stark_proof_str: string): StwoResult;

export function verify_stark_proof_poseidon(log_n_instances: number, stark_proof_str: string): StwoResult;

export function verify_stark_proof_wide_fibo(log_fibonacci_size: number, log_n_instances: number, stark_proof_str: string): StwoResult;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly stark_proof_wide_fibo: (a: number, b: number) => number;
    readonly verify_stark_proof_wide_fibo: (a: number, b: number, c: number, d: number) => number;
    readonly stark_proof_multi_fibo: (a: number, b: number, c: number, d: number) => number;
    readonly verify_stark_proof_multi_fibo: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly prove_stark_proof_poseidon: (a: number) => number;
    readonly verify_stark_proof_poseidon: (a: number, b: number, c: number) => number;
    readonly __wbg_stworesult_free: (a: number, b: number) => void;
    readonly stworesult_success: (a: number) => number;
    readonly stworesult_message: (a: number, b: number) => void;
    readonly prove_and_verify: (a: number) => number;
    readonly verify_stark_proof: (a: number, b: number, c: number) => number;
    readonly prove_and_verify_fib: (a: number, b: number) => number;
    readonly verify_stark_proof_fib: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
