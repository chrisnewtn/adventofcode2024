import { getInput } from "./lib.js";

const {left, right} = await getInput();

const scores = new Map();
const totals = new Map();

for (const l of left) {
  if (scores.has(l)) {
    const score = scores.get(l);
    const total = totals.get(l);
    total.set(l, total + score);
    continue;
  }
  const occurances = right.filter(r => l === r).length;

  scores.set(l, l * occurances);
  totals.set(l, scores.get(l));
}

let total = 0;

for (const runningTotal of totals.values()) {
  total += runningTotal;
}

console.log(total);
