
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
imports['env'] = require('env');
let wasm;
const { TextDecoder } = require(`util`);

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

module.exports.__wbg_getTime_46267b1c24877e30 = function(arg0) {
    const ret = arg0.getTime();
    return ret;
};

module.exports.__wbg_getTimezoneOffset_6b5752021c499c47 = function(arg0) {
    const ret = arg0.getTimezoneOffset();
    return ret;
};

module.exports.__wbg_new0_f788a2397c7ca929 = function() {
    const ret = new Date();
    return ret;
};

module.exports.__wbg_new_31a97dac4f10fab7 = function(arg0) {
    const ret = new Date(arg0);
    return ret;
};

module.exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

module.exports.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return ret;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

const path = require('path').join(__dirname, 'cal_lock_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

wasm.__wbindgen_start();

