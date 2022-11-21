import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((x) => parseInt(x));
};

const f0 = (input) => {
  return input.reduce((a, b) => a + b);
};

const f1 = (input) => {
  let seen = new Set();
  let acc = 0;
  let index = 0;

  while (true) {
    if (seen.has(acc)) {
      return acc;
    }
    seen.add(acc);
    acc += input[index];

    index = (index + 1) % input.length;
  }

  return undefined;
};

test([]);

benchmark(f0, f1);
