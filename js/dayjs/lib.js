import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines;
};

const f0 = (input) => {
  return 0;
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 0 },
  { f: f1, expected: 0 },
]);

// benchmark(f0, f1);
