import { benchmark, test } from "./main.js";

export const format = (lines) => {
  let groups = [];
  let current_group = [];
  lines.forEach((line) => {
    if (line.trim() == "") {
      groups.push(current_group);
      current_group = [];
    } else {
      current_group.push(Array.from(line));
    }
  });

  if (current_group.length > 0) {
    groups.push(current_group);
  }

  return groups;
};

const count_letters_in_group0 = (group) => {
  let letters = [];
  group.forEach((ind) => {
    ind.forEach((c) => letters.push(c));
  });
  return new Set(letters).size;
};

const f0 = (groups) => {
  return groups.map(count_letters_in_group0).reduce((a, b) => a + b, 0);
};

const count_letters_in_group1 = (group) => {
  let letters = group[0];
  group.forEach((ind) => {
    letters = letters.filter((c) => ind.includes(c));
  });
  return new Set(letters).size;
};

const f1 = (groups) => {
  return groups.map(count_letters_in_group1).reduce((a, b) => a + b, 0);
};

test([
  { f: f0, expected: 11 },
  { f: f1, expected: 6 },
]);

benchmark(f0, f1);
