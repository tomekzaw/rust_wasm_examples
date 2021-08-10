async function main() {
  const wasm = await import("../pkg/index.js");
  const f = wasm.make_f();
  console.log(f(123));
  console.log(f(123));
  console.log(f(123));
}

main();
