import { benchmark, test } from "./main.js";

const f0 = (input) => {
  for (let i = 0; i < input.length; i++) {
    let v0 = input[i];
    for (let j = i + 1; j < input.length; j++) {
      if (v0 + input[j] == 2020) {
        return v0 * input[j];
      }
    }
  }
};

const f1 = (input) => {
  for (let i = 0; i < input.length; i++) {
    let v0 = input[i];
    for (let j = i + 1; j < input.length; j++) {
      let v1 = input[j];
      for (let k = j + 1; k < input.length; k++) {
        if (v0 + input[j] + input[k] == 2020) {
          return v0 * v1 * input[k];
        }
      }
    }
  }
};

test(f0, f1, 514579, 241861950);
benchmark(f0, f1);
