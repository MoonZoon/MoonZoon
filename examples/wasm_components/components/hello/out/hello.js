let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const utf8Decoder = new TextDecoder();

export async function instantiate(compileCore, imports, instantiateCore = WebAssembly.instantiate) {
  const module0 = compileCore('hello.core.wasm');
  
  let exports0;
  let memory0;
  let postReturn0;
  
  ({ exports: exports0 } = await instantiateCore(await module0));
  memory0 = exports0.memory;
  postReturn0 = exports0['cabi_post_say-something'];
  return {
    saySomething() {
      const ret = exports0['say-something']();
      const ptr0 = dataView(memory0).getInt32(ret + 0, true);
      const len0 = dataView(memory0).getInt32(ret + 4, true);
      const result0 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr0, len0));
      postReturn0(ret);
      return result0;
    },
  };
}
