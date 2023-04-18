import * as fs from "fs";

const fileName = process.argv[2];

fs.readFileSync(fileName)
  .toString()
  .split("\n")
  .forEach((line) => {
    let print = parseInt(line);
    if (isNaN(print)) {
      console.log("Line not a number");
    } else {
      console.log(print);
    }
  });
