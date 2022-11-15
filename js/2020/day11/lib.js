import { benchmark, test } from "./main.js";

// floor : 0
// free : 1
// occupied : 2

export const format = (lines) => {
  const get_identifier = (c) => {
    switch (c) {
      case "L":
        return 1;
      case "#":
        return 2;
      case ".":
        return 0;
      default:
        raise`wrong input : ${c}`;
    }
  };
  return lines.map((l) => Array.from(l).map(get_identifier));
};

const next_state = (i, j, grid) => {};

const f0 = (input) => {
  return 0;
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 37 },
  { f: f1, expected: 0 },
]);

benchmark(f0, f1);
