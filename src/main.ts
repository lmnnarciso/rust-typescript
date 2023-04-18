// let arr = [1, 2, 3];

// for (const x of arr) {
//   console.log(`${x + 1}`);
// }

var fs = require("fs");

var data = fs.readFileSync("lines", "utf8");
data = data
  .split("\n")
  .filter((_: any, index: number) => index % 2 === 0)
  .map((x: string) => console.log(x));
