import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines[0].split(",").map((x) => parseInt(x));
};

const run_instrs = (instrs) => {
  let i = 0;
  while (true) {
    let cmd = instrs[i];
    switch (cmd) {
      case 1:
        instrs[instrs[i + 3]] = instrs[instrs[i + 2]] + instrs[instrs[i + 1]];
        i += 4;
        break;

      case 2:
        instrs[instrs[i + 3]] = instrs[instrs[i + 2]] * instrs[instrs[i + 1]];
        i += 4;
        break;

      case 99:
        return 0;

      default:
        return instrs[i];
    }
  }
};

const f0 = (input) => {
  input[1] = 12;
  input[2] = 2;
  run_instrs(input);
  return input[0];
};

const f1 = (input) => {
  for (let i = 0; i < 99; i++) {
    for (let j = 0; j < 99; j++) {
      let clone = structuredClone(input);
      clone[1] = i;
      clone[2] = j;
      run_instrs(clone);
      if (clone[0] === 19690720) {
        return 100 * i + j;
      }
    }
  }
};

test([]);

benchmark(f0, f1);
