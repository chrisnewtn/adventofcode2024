import { getInput } from "./lib.js";

const {left, right} = await getInput();

left.sort();
right.sort();

const answer = left.reduce((memo, n, i) => memo + Math.abs(n - right[i]), 0);

console.log(answer);
