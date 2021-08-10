async function main() {
  const wasm = await import("../pkg/index.js");
  wasm.foo();
}

main();
