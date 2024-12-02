import { createReadStream } from 'node:fs';
import readline from 'node:readline/promises';

/**
 * @param {number[]} report
 */
function isReportSafe(report) {
  let isSafe = true;
  let isIncrease = null;

  for (let i = 0; i < report.length; i++) {
    const curr = report[i];
    const next = report[i + 1];

    const diff = curr - next;

    if (diff === 0 || Math.abs(diff) > SAFE_CHANGE_MAX) {
      isSafe = false;
      break;
    }

    if (isIncrease === null) {
      isIncrease = diff > 0;
      continue;
    }

    if (diff > 0 && !isIncrease) {
      isSafe = false;
      break;
    } else if (diff < 0 && isIncrease) {
      isSafe = false;
      break;
    }
  }

  return isSafe;
}

const input = createReadStream(new URL('./input', import.meta.url), 'utf8');

const SAFE_CHANGE_MAX = 3;
let numberOfSafeReports = 0;

for await (const line of readline.createInterface({input})) {
  const report = line.split(/\s+/).map(n => parseInt(n, 10));

  if (isReportSafe(report)) {
    numberOfSafeReports++;
  }
}

console.log(numberOfSafeReports);
