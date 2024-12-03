import fs from 'node:fs/promises';

const input = await fs.readFile(new URL('./input', import.meta.url), 'utf8');

const matcher = /mul\((?<left>\d+),(?<right>\d+)\)/g;

let total = 0;

for (const {groups: {left, right}} of input.matchAll(matcher)) {
  total += parseInt(left, 10) * parseInt(right, 10);
}

console.log(total);
