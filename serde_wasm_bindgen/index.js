async function main() {
  const wasm = await import("../pkg/index.js");

  const str =
    '{"address":{"city":"Example City","country":"Germany","street":"Example Street"},"age":51,"name":"Bob","siblings":["Alice","Joe"]}';
  console.log(str);

  const obj = wasm.parse(str);
  console.log(obj);

  const str2 = wasm.stringify(obj);
  console.log(str2);
}

main();
