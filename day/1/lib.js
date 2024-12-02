import { createReadStream } from 'node:fs';
import readline from 'node:readline/promises';

export async function getInput() {
  const input = createReadStream(new URL('./input', import.meta.url), 'utf8');

  const left = [];
  const right = [];

  for await (const line of readline.createInterface({input})) {
    const [l, r] = line.split(/\s+/);

    left.push(parseInt(l, 10));
    right.push(parseInt(r, 10));
  }

  return {left, right};
}
