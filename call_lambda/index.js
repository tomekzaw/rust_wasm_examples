async function main() {
  const wasm = await import("../pkg/index.js.js");
  const f = () => 3.0;
  const result = wasm.call_lambda(f);
  console.log(result);
}

main();
