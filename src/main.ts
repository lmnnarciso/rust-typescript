// let arr = [1, 2, 3];

// for (const x of arr) {
//   console.log(`${x + 1}`);
// }

var fs = require("fs");

var data = fs.readFileSync("lines", "utf8");
data = data.split("\n").map((x: string) => console.log(x));
