import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return Array.from(lines[0]);
};

const simplify = (data) => {
  let chars = data.chars;
  let changed = false;
  let newString = [];
  let index = 0;
  while (index < chars.length) {
    if (
      index < chars.length - 1 &&
      ((chars[index].toUpperCase() == chars[index] &&
        chars[index].toLowerCase() == chars[index + 1]) ||
        (chars[index].toLowerCase() == chars[index] &&
          chars[index].toUpperCase() == chars[index + 1]))
    ) {
      changed = true;
      index += 2;
    } else {
      newString.push(chars[index]);
      index++;
    }
  }

  data.chars = newString;

  return changed;
};

const f0 = (input) => {
  let data = { chars: input };
  while (simplify(data));
  return data.chars.length;
};

const remove_polymer = (data, c) => {
  data.chars = data.chars.filter((x) => x != c && x != c.toUpperCase());
};

const polymer_cost = (data, c) => {
  let data0 = structuredClone(data);
  remove_polymer(data0, c);
  while (simplify(data0));
  return data0.chars.length;
};

const min_reaction = (input, char_list) => {
  let data = { chars: input };
  return char_list
    .map((c) => polymer_cost(data, c))
    .reduce((a, b) => (a > b ? b : a), Number.MAX_SAFE_INTEGER);
};

const f1 = (input) => {
  const alphabet = Array.from({ length: 25 }, (x, i) => i).map((i) =>
    String.fromCharCode(97 + i)
  );
  return min_reaction(input, alphabet);
};

test([
  { f: f0, expected: 10 },
  { f: (input) => min_reaction(input, ["a", "b", "c", "d"]), expected: 4 },
]);

benchmark(f0, f1);
