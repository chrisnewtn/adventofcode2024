import fs from 'node:fs/promises';

const input = await fs.readFile(new URL('./input', import.meta.url), 'utf8');

// failed attempt at solving it with a regex:
// const matcher = /((?<!don't\(\).*)mul\((?<l1>\d+),(?<r1>\d+)\)|(?<=do\(\).*)mul\((?<l2>\d+),(?<r2>\d+)\))/g;

/**
 * in a genre of answer known as:
 *
 * hideous
 *
 * ...but works.
 */

const isDigit = /\d/;
const mul = /mul\((\d+),(\d+)\)/;
let seenDo = true;
let total = 0;
let buffer = '';

for (const char of input) {
  if (seenDo) {
    if (
      buffer === '' && char === 'm' ||
      buffer === 'm' && char === 'u' ||
      buffer === 'mu' && char === 'l' ||
      buffer === 'mul' && char === '('
    ) {
      buffer += char;
      continue;
    }
    if (
      buffer === '' && char === 'd' ||
      buffer === 'd' && char === 'o' ||
      buffer === 'do' && char === 'n' ||
      buffer === 'don' && char === '\'' ||
      buffer === 'don\'' && char === 't' ||
      buffer === 'don\'t' && char === '('
    ) {
      buffer += char;
      continue;
    }
    if (buffer + char === 'don\'t()') {
      seenDo = false;
      buffer = '';
      continue;
    }
    if (buffer.startsWith('mul(')) {
      if (isDigit.test(char) || char === ',') {
        buffer += char;
        continue;
      }
      if (char === ')') {
        const operation = buffer + char;
        buffer = '';

        if (mul.test(operation)) {
          const [_, left, right] = operation.match(mul);
          total += parseInt(left, 10) * parseInt(right, 10);
        }
      }
    }
  } else {
    if (
      buffer === '' && char === 'd' ||
      buffer === 'd' && char === 'o' ||
      buffer === 'do' && char === '('
    ) {
      buffer += char;
      continue;
    }
    if (buffer + char === 'do()') {
      seenDo = true;
    }
  }

  buffer = '';
}

console.log(total);
