import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((x) => parseInt(x));
};

const test_index = (index, length, nums) => {
  let goal = nums[index];
  for (let i = 0; i < length; i++) {
    for (let j = i + 1; j < length; j++) {
      if (nums[index - length + i] + nums[index - length + j] === goal) {
        return true;
      }
    }
  }

  return false;
};

const first_wrong_number = (length, nums) => {
  for (let i = length; i < nums.length; i++) {
    if (!test_index(i, length, nums)) {
      return nums[i];
    }
  }

  return null;
};

const f0 = (input) => {
  return first_wrong_number(25, input);
};

const find_subsum = (goal, nums) => {
  let down = 0;
  let top = 0;
  let sum = 0;

  while (true) {
    if (sum < goal && top == nums.length - 1) {
      break;
    }

    if (sum == goal) {
      return [down, top];
    } else if (sum < goal) {
      sum += nums[top++];
    } else {
      sum -= nums[down++];
    }
  }

  return null;
};

const f1 = (input) => {
  let [down, top] = find_subsum(first_wrong_number(25, input), input);
  let slice = input.slice(down, top);
  return Math.max(...slice) + Math.min(...slice);
};

const test_f1 = (input) => {
  let [down, top] = find_subsum(first_wrong_number(5, input), input);
  let slice = input.slice(down, top);
  return Math.max(...slice) + Math.min(...slice);
};

test([
  { f: (input) => first_wrong_number(5, input), expected: 127 },
  { f: test_f1, expected: 62 },
]);

benchmark(f0, f1);
