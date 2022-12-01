import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return lines[0].split(",").map((x) => parseInt(x));
};

const word_index0 = 2020;

const get_nth_spoken_number = (nums, word_index) => {
  let i = nums.length - 1;
  let last_indexes: Map<number, number> = new Map<number, number>();
  for (let index = 0; index < nums.length - 1; index++) {
    last_indexes.set(nums[index], index);
  }
  let last_spoken = nums[nums.length - 1];

  while (i < word_index - 1) {
    let temp = last_spoken;

    let last_index = last_indexes.get(last_spoken);
    if (last_index === undefined) {
      last_spoken = 0;
    } else {
      last_spoken = i - last_index;
    }

    last_indexes.set(temp, i);
    i++;
  }

  return last_spoken;
};

const f0 = (nums) => {
  return get_nth_spoken_number(nums, word_index0);
};

const word_index1 = 30000000;

const f1 = (nums) => {
  return get_nth_spoken_number(nums, word_index1);
};

test([
  { f: f0, expected: 436 },
  { f: f0, expected: 1836, file: "test_input1.txt" },
  { f: f1, expected: 175594 },
]);

benchmark(f0, f1);
