async function main() {
  const wasm = await import("../pkg/index.js");

  const str =
    '{"address":{"city":"Example City","country":"Germany","street":"Example Street"},"age":51,"name":"Bob","siblings":["Alice","Joe"]}';
  console.log(str);

  const obj = wasm.serialize(str);
  console.log(obj);

  const str2 = wasm.parse(obj);
  console.log(str2);
}

main();
