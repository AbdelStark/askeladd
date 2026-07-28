/* @ts-self-types="./stwo_wasm.d.ts" */

/**
 * Result type handed back to JavaScript: a success flag plus a message
 * (the serialized proof on success, the error otherwise).
 */
export class StwoResult {
    static __wrap(ptr) {
        const obj = Object.create(StwoResult.prototype);
        obj.__wbg_ptr = ptr;
        StwoResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StwoResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stworesult_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.stworesult_message(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.stworesult_success(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) StwoResult.prototype[Symbol.dispose] = StwoResult.prototype.free;

/**
 * @param {number} log_n_instances
 * @returns {StwoResult}
 */
export function prove_and_verify(log_n_instances) {
    const ret = wasm.prove_and_verify(log_n_instances);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_size
 * @param {number} claim
 * @returns {StwoResult}
 */
export function prove_and_verify_fib(log_size, claim) {
    const ret = wasm.prove_and_verify_fib(log_size, claim);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_n_instances
 * @returns {StwoResult}
 */
export function prove_stark_proof_poseidon(log_n_instances) {
    const ret = wasm.prove_stark_proof_poseidon(log_n_instances);
    return StwoResult.__wrap(ret);
}

/**
 * @param {Uint32Array} log_sizes
 * @param {Uint32Array} claims_int
 * @returns {StwoResult}
 */
export function stark_proof_multi_fibo(log_sizes, claims_int) {
    const ptr0 = passArray32ToWasm0(log_sizes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray32ToWasm0(claims_int, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.stark_proof_multi_fibo(ptr0, len0, ptr1, len1);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_fibonacci_size
 * @param {number} log_n_instances
 * @returns {StwoResult}
 */
export function stark_proof_wide_fibo(log_fibonacci_size, log_n_instances) {
    const ret = wasm.stark_proof_wide_fibo(log_fibonacci_size, log_n_instances);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_n_instances
 * @param {string} stark_proof_str
 * @returns {StwoResult}
 */
export function verify_stark_proof(log_n_instances, stark_proof_str) {
    const ptr0 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof(log_n_instances, ptr0, len0);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_size
 * @param {number} claim
 * @param {string} stark_proof_str
 * @returns {StwoResult}
 */
export function verify_stark_proof_fib(log_size, claim, stark_proof_str) {
    const ptr0 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof_fib(log_size, claim, ptr0, len0);
    return StwoResult.__wrap(ret);
}

/**
 * @param {Uint32Array} log_sizes
 * @param {Uint32Array} claims_int
 * @param {string} stark_proof_str
 * @returns {StwoResult}
 */
export function verify_stark_proof_multi_fibo(log_sizes, claims_int, stark_proof_str) {
    const ptr0 = passArray32ToWasm0(log_sizes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray32ToWasm0(claims_int, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof_multi_fibo(ptr0, len0, ptr1, len1, ptr2, len2);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_n_instances
 * @param {string} stark_proof_str
 * @returns {StwoResult}
 */
export function verify_stark_proof_poseidon(log_n_instances, stark_proof_str) {
    const ptr0 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof_poseidon(log_n_instances, ptr0, len0);
    return StwoResult.__wrap(ret);
}

/**
 * @param {number} log_fibonacci_size
 * @param {number} log_n_instances
 * @param {string} stark_proof_str
 * @returns {StwoResult}
 */
export function verify_stark_proof_wide_fibo(log_fibonacci_size, log_n_instances, stark_proof_str) {
    const ptr0 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof_wide_fibo(log_fibonacci_size, log_n_instances, ptr0, len0);
    return StwoResult.__wrap(ret);
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_log_723d902a97d7992a: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
    };
    return {
        __proto__: null,
        "./stwo_wasm_bg.js": import0,
    };
}

const StwoResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stworesult_free(ptr, 1));

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('stwo_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
