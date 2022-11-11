import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((line) => Array.from(line).map((c) => c == "#"));
};

let iterate = (state, input, xIter, yIter) => {
  if (input[state.y][state.x % input[0].length]) {
    state.count++;
  }
  state.x += xIter;
  state.y += yIter;

  return state.y < input.length;
};

let count_trees = (xIter, yIter, input) => {
  let state = { x: 0, y: 0, count: 0 };
  while (iterate(state, input, xIter, yIter)) {}
  return state.count;
};

const f0 = (input) => {
  return count_trees(3, 1, input);
};

const f1 = (input) => {
  return [
    [1, 1],
    [3, 1],
    [5, 1],
    [7, 1],
    [1, 2],
  ]
    .map(([xIter, yIter]) => count_trees(xIter, yIter, input))
    .reduce((a, b) => a * b);
};

test(f0, f1, 7, 336);
benchmark(f0, f1);
