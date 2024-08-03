let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {number} log_n_instances
* @returns {StwoResult}
*/
export function prove_stark_proof_poseidon(log_n_instances) {
    const ret = wasm.prove_stark_proof_poseidon(log_n_instances);
    return StwoResult.__wrap(ret);
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

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
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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
* @returns {StwoResult}
*/
export function stark_proof_wide_fibo(log_fibonacci_size, log_n_instances) {
    const ret = wasm.stark_proof_wide_fibo(log_fibonacci_size, log_n_instances);
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

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}
/**
* @param {number} log_size
* @param {number} claim
* @returns {StwoResult}
*/
export function prove_and_verify(log_size, claim) {
    const ret = wasm.prove_and_verify(log_size, claim);
    return StwoResult.__wrap(ret);
}

/**
* @param {number} log_size
* @param {number} claim
* @param {string} stark_proof_str
* @returns {StwoResult}
*/
export function verify_stark_proof(log_size, claim, stark_proof_str) {
    const ptr0 = passStringToWasm0(stark_proof_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.verify_stark_proof(log_size, claim, ptr0, len0);
    return StwoResult.__wrap(ret);
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
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

const StwoResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stworesult_free(ptr >>> 0));
/**
*/
export class StwoResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
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
        wasm.__wbg_stworesult_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    get success() {
        const ret = wasm.stworesult_success(this.__wbg_ptr);
        return ret !== 0;
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
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
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
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_b6f0d1d33b5b64be = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('stwo_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
