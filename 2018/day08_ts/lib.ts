import { benchmark, test } from "./main.js";

export const format = (lines: string[]): number[] => {
  return lines[0].split(" ").map((x) => parseInt(x));
};

const metadata_sum = (input: number[], index: number = 0): [number, number] => {
  let sum = 0;
  const child_count = input[index];
  const metadata_count = input[index + 1];
  index += 2;
  for (let i = 0; i < child_count; i++) {
    let [partial_sum, new_index] = metadata_sum(input, index);
    index = new_index;
    sum += partial_sum;
  }

  for (let _ = 0; _ < metadata_count; _++) {
    sum += input[index];
    index++;
  }

  return [sum, index];
};

const f0 = (input) => {
  return metadata_sum(input)[0];
};

const node_value = (input: number[], index: number = 0): [number, number] => {
  let sum = 0;
  const child_count = input[index];
  const metadata_count = input[index + 1];
  index += 2;
  let childs_sums = new Array(child_count).fill(null);

  for (let i = 0; i < child_count; i++) {
    let [partial_sum, new_index] = node_value(input, index);
    index = new_index;
    childs_sums[i] = partial_sum;
  }

  if (child_count === 0) {
    for (let _ = 0; _ < metadata_count; _++) {
      sum += input[index];
      index++;
    }
  } else {
    for (let _ = 0; _ < metadata_count; _++) {
      let pointer = input[index];
      if (pointer > 0 && pointer <= child_count) {
        sum += childs_sums[pointer - 1];
      }
      index++;
    }
  }

  return [sum, index];
};

const f1 = (input) => {
  return node_value(input)[0];
};

test([
  { f: f0, expected: 138 },
  { f: f1, expected: 66 },
]);

benchmark(f0, f1);
