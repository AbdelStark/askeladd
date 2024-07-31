/* tslint:disable */
/* eslint-disable */
/**
* @param {number} log_size
* @param {number} claim
* @returns {StwoResult}
*/
export function prove_and_verify(log_size: number, claim: number): StwoResult;
/**
* @param {number} log_size
* @param {number} claim
* @param {string} stark_proof_str
* @returns {StwoResult}
*/
export function verify_stark_proof(log_size: number, claim: number, stark_proof_str: string): StwoResult;
/**
*/
export class StwoResult {
  free(): void;
/**
*/
  readonly message: string;
/**
*/
  readonly success: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_stworesult_free: (a: number) => void;
  readonly stworesult_success: (a: number) => number;
  readonly stworesult_message: (a: number, b: number) => void;
  readonly prove_and_verify: (a: number, b: number) => number;
  readonly verify_stark_proof: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
