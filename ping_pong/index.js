async function main() {
  const wasm = await import("../../pkg/index.js");
  console.log(wasm.ping());
}

main();
