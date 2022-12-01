import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines[0].split("-").map((x) => parseInt(x));
};

const is_valid0 = (n) => {
  let chars = Array.from(n.toString()).map((x) => parseInt(x));
  let found_2_same_digits = false;
  for (let i = 0; i < chars.length - 1; i++) {
    if (chars[i] == chars[i + 1]) {
      found_2_same_digits = true;
    }

    if (chars[i] > chars[i + 1]) {
      return false;
    }
  }

  if (!found_2_same_digits) {
    return false;
  }

  return true;
};

const f0 = ([min, max]) => {
  return Array.from({ length: max - min + 1 }, (x, i) => min + i)
    .map(is_valid0)
    .reduce((a, b) => a + b, 0);
};

const is_valid1 = (n) => {
  let chars = Array.from(n.toString()).map((x) => parseInt(x));
  let found_2_same_digits = false;
  for (let i = 0; i < chars.length - 1; i++) {
    if (
      chars[i] == chars[i + 1] &&
      (i == 0 || chars[i - 1] != chars[i]) &&
      (i == chars.length - 2 || chars[i + 2] != chars[i])
    ) {
      found_2_same_digits = true;
    }

    if (chars[i] > chars[i + 1]) {
      return false;
    }
  }

  if (!found_2_same_digits) {
    return false;
  }

  return true;
};

const f1 = ([min, max]) => {
  return Array.from({ length: max - min + 1 }, (x, i) => min + i)
    .map(is_valid1)
    .reduce((a, b) => a + b, 0);
};

// test([
//   { f: f0, expected: 0 },
//   { f: f1, expected: 0 },
// ]);

benchmark(f0, f1);
