async function main() {
  const wasm = await import("../pkg/index.js");
  const blob = new Blob(["Hello world!"]);
  console.log(wasm.isblob(blob));
  console.log(wasm.isblob(123));
  console.log(wasm.isblob("foo"));
}

main();
