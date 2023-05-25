export async function loadWasm() {
    const memory = new WebAssembly.Memory({ initial: 64 });
    const imports = { env: { memory } };
    const resp = await fetch('vulgar_fractions.wasm');
    const bytes = await resp.arrayBuffer();
    const source = await WebAssembly.instantiate(bytes, imports);
    return { exports: source.instance.exports, memory };
}
