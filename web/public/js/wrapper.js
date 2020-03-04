export function curryVulgarFraction(exports, memory) {
    const vulgarFractionWasm = wrapWasmStringFunction(
        exports.vulgar_fraction,
        exports.vulgar_fraction_len,
        exports.vulgar_fraction_free,
        memory,
    );
    return (nominator, denominator) => vulgarFractionWasm(nominator, denominator);
}

function wrapWasmStringFunction(fn, getLength, freeString, memory) {
    return (...args) => {
        const pointer = fn(...args);
        const length = getLength(pointer);
        const rawString = new Uint8Array(memory.buffer, pointer, length);
        const decoder = new TextDecoder();
        const string = decoder.decode(rawString);
        freeString(pointer);
        return string;
    };
}
