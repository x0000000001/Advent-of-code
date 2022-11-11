import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((line) => {
    let words = line.split(" ");
    let min_max = words[0].split("-").map((v) => parseInt(v));
    return {
      min: min_max[0],
      max: min_max[1],
      letter: words[1].charAt(0),
      password: words[2],
    };
  });
};

const is_valid0 = ({ min, max, letter, password }) => {
  let count = Array.from(password).filter((c) => c == letter).length;
  return count >= min && count <= max;
};

const f0 = (input) => {
  return input.filter(is_valid0).length;
};

const is_valid1 = ({ min, max, letter, password }) => {
  let isPresent0 = password.charAt(min - 1) == letter;
  let isPresent1 = password.charAt(max - 1) == letter;
  return (!isPresent0 && isPresent1) || (isPresent0 && !isPresent1);
};

const f1 = (input) => {
  return input.filter(is_valid1).length;
};

test(f0, f1, 2, 1);
benchmark(f0, f1);
