// let arr = [1, 2, 3];

// for (const x of arr) {
//   console.log(`${x + 1}`);
// }

type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push("hello fem!");
}
const item: Item[] = [];

const numbers: number[] = [];

append(numbers);
