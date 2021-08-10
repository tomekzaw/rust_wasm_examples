async function main() {
  const wasm = await import("../pkg/index.js");
  const result = await wasm.foo();
  console.log(result);
}

main();
