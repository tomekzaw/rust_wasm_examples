async function main() {
  const wasm = await import("../pkg/index.js");

  const blob = new Blob(["Hello world!"]);
  const obj = {
    a: 42,
    b: "foo",
    c: blob,
    d: {
      dd: {
        ddd: blob,
      },
    },
  };
  const result = wasm.json_stringify_with_blobs(obj);
  console.log(result);
}

main();
