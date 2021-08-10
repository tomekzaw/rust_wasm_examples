function measure_sync(name, func) {
  const start = performance.now();
  const result = func();
  const end = performance.now();
  console.log(`${name}: ${end - start} ms`);
  console.log(result);
  return result;
}

async function measure_async(name, func) {
  const start = performance.now();
  const result = await func();
  const end = performance.now();
  console.log(`${name}: ${end - start} ms`);
  console.log(result);
  return result;
}

async function main() {
  const wasm = await import("../pkg/index.js");

  // measure_sync("make_array_buffer_box_u8", wasm.make_array_buffer_box_u8); // this is slow

  measure_sync("make_array_buffer_web_sys", wasm.make_array_buffer_web_sys);

  const ab = wasm.make_array_buffer_web_sys();

  measure_sync("new Uint8Array(ab)", () => new Uint8Array(ab));

  const ta = new Uint8Array(ab);

  measure_sync("new Blob([ta])", () => new Blob([ta]));

  measure_sync("new Blob([ab])", () => new Blob([ab]));

  const blob = new Blob([ab]);

  measure_sync("make_blob_with_buffer", wasm.make_blob_with_buffer);

  measure_sync("make_blob_with_u8_array", wasm.make_blob_with_u8_array);

  measure_sync("take_uint8array", () => wasm.take_uint8array(ta));

  await measure_async("take_blob", async () => wasm.take_blob(blob));

  await measure_async(
    "await blob.arrayBuffer(), new Uint8Array(ab), take_uint8array",
    async () => {
      const ab = await blob.arrayBuffer();
      const ta = new Uint8Array(ab);
      return wasm.take_uint8array(ta);
    }
  );
}

main();
