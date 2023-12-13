import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines;
};

const contains_any_letter_n_times = (n, word) => {
  let d = {};

  Array.from(word).forEach((letter) => {
    if (!(letter in d)) {
      d[letter] = 1;
    } else {
      d[letter]++;
    }
  });

  for (const key in d) {
    if (d[key] === n) {
      return true;
    }
  }

  return false;
};

const count_word_with_n_occurences = (n, words) => {
  return words
    .map((word) => contains_any_letter_n_times(n, word))
    .reduce((a, b) => a + b, 0);
};

const f0 = (input) => {
  return (
    count_word_with_n_occurences(2, input) *
    count_word_with_n_occurences(3, input)
  );
};

const common_letters = (w0, w1) => {
  return Array.from(w0)
    .map((c, i) => {
      if (c === w1.charAt(i)) {
        return c;
      } else {
        return null;
      }
    })
    .filter((c) => c !== null)
    .join("");
};

const f1 = (input) => {
  for (let i = 0; i < input.length; i++) {
    const w0 = input[i];
    for (let j = i + 1; j < input.length; j++) {
      const w1 = input[j];

      let cl = common_letters(w0, w1);

      if (cl.length === w0.length - 1) {
        return cl;
      }
    }
  }

  return null;
};

test([
  { f: f0, expected: 12 },
  { f: f1, expected: "fgij", file: "test_input1.txt" },
]);

benchmark(f0, f1);
