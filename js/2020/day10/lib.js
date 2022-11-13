import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((x) => parseInt(x));
};

const count_occurences = (x, arr) => {
  return arr.reduce((acc, a) => (a == x ? acc + 1 : acc), 0);
};

const sort_chargers = (chargers) => {
  chargers = chargers.sort((a, b) => a - b);
  chargers.push(chargers[chargers.length - 1] + 3);
  chargers.unshift(0);
};

const f0 = (input) => {
  sort_chargers(input);

  let differences = [...Array(input.length - 1).keys()].map(
    (i) => input[i + 1] - input[i]
  );

  return count_occurences(1, differences) * count_occurences(3, differences);
};

const compute_arrangements = (chargers, beginning_charger_index, occ) => {
  if (beginning_charger_index == chargers.length - 1) {
    return 1;
  }

  let sum = 0;
  for (
    let index = beginning_charger_index + 1;
    index < chargers.length &&
    chargers[index] - chargers[beginning_charger_index] < 4;
    index++
  ) {
    if (!(index in occ)) {
      occ[index] = compute_arrangements(chargers, index, occ);
    }

    sum += occ[index];
  }

  return sum;
};

const f1 = (input) => {
  sort_chargers(input);
  return compute_arrangements(input, 0, {});
};

test([
  { f: f0, expected: 35, file: "test_input0.txt" },
  { f: f0, expected: 220, file: "test_input1.txt" },
  { f: f1, expected: 8, file: "test_input0.txt" },
  { f: f1, expected: 19208, file: "test_input1.txt" },
]);

benchmark(f0, f1);
