export function bar() {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      resolve("bar");
    }, 1000);
  });
}
