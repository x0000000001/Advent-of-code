import { benchmark, test } from "./main.js";

type Rule =
  | { atomic: true; val: "a" }
  | { atomic: true; val: "b" }
  | { atomic: false; possibilities: number[][] };

export const format = (lines: string[]) => {
  let rules: { [key: number]: Rule } = {};
  let i = 0;
  while (true) {
    let words = lines[i].split(": ");
    if (words.length === 1) {
      break;
    }

    const id = parseInt(words[0]);

    if (words[1].charAt(0) === '"') {
      switch (words[1]) {
        case '"a"':
          rules[id] = { atomic: true, val: "a" };
          break;
        case '"b"':
          rules[id] = { atomic: true, val: "b" };
          break;
        default:
          throw `unrecognized : ${words[1]}`;
      }
    } else {
      rules[id] = {
        atomic: false,
        possibilities: words[1]
          .split(" | ")
          .map((p) => p.split(" ").map((x) => parseInt(x))),
      };
    }

    i++;
  }
  let texts = [];

  while (i < lines.length) {
    texts.push(lines[i]);
    i++;
  }

  return [rules, texts];
};

const get_possible_sols = (
  s: string,
  rule_index,
  rules: { [key: number]: Rule }
): string[] => {
  let r = rules[rule_index];

  if (r.atomic === true) {
    if (s.startsWith(r.val)) {
      // console.log(s, s.substring(r.val.length));
      return [s.substring(r.val.length)];
    } else {
      return [];
    }
  }

  return r.possibilities.flatMap((p) => {
    let temp_possible_sols = [structuredClone(s)];
    for (let i = 0; i < p.length; i++) {
      temp_possible_sols = temp_possible_sols.flatMap((s1) =>
        get_possible_sols(s1, p[i], rules)
      );
    }
    return temp_possible_sols;
  });
};

const is_valid = (s: string, rules): boolean => {
  return get_possible_sols(s, 0, rules).includes("");
};

const f0 = ([rules, texts]) => {
  return texts
    .map((t) => is_valid(t, rules))
    .reduce((a, b) => a + (b ? 1 : 0), 0);
};

const f1 = ([rules, texts]) => {
  rules[8] = { atomic: false, possibilities: [[42], [42, 8]] };
  rules[11] = {
    atomic: false,
    possibilities: [
      [42, 31],
      [42, 11, 31],
    ],
  };

  return texts
    .map((t) => is_valid(t, rules))
    .reduce((a, b) => a + (b ? 1 : 0), 0);
};

test([
  { f: f0, expected: 2 },
  { f: f1, expected: 12, file: "test_input1.txt" },
]);

benchmark(f0, f1);
