import { benchmark, test } from "./main.js";

type InputType = { [index: number]: number };

export const format = (lines: string[]): InputType => {
  let instrs: InputType = {};
  lines[0].split(",").forEach((c, i) => (instrs[i] = parseInt(c)));
  return instrs;
};

/**
 *
 * @param instrs instrs to be executed
 * @param inputs inputs to feed to the programm
 * @param i beginning instruction index
 * @param relative_base relative base as defined for type 2 values
 * @returns index of last instruction (null if done), outputs, relative_base value
 */
const run_instrs = (
  instrs: InputType,
  inputs: number[],
  i = 0,
  relative_base = 0
): [number, number[], number] => {
  let output = [];
  let input_index = 0;
  while (true) {
    let cmd = instrs[i] | 0;
    let cmd_string: string = cmd.toString();
    let parameters = [null, null, null];
    for (let j = 0; j < 3; j++) {
      let mode = 0;
      if (cmd_string.length > 2 + j) {
        mode = parseInt(cmd_string.charAt(cmd_string.length - (3 + j)));
      }
      let val = instrs[i + 1 + j] | 0;
      switch (mode) {
        case 0:
          parameters[j] = instrs[val] | 0;
          break;
        case 1:
          parameters[j] = val;
          break;
        case 2:
          parameters[j] = instrs[relative_base + val] | 0;
          break;
      }
    }
    cmd = cmd % 100;
    switch (cmd) {
      case 1:
        instrs[instrs[i + 3] | 0] = parameters[0] + parameters[1];
        i += 4;
        break;

      case 2:
        instrs[instrs[i + 3] | 0] = parameters[0] * parameters[1];
        i += 4;
        break;

      case 99:
        return [null, output, relative_base];

      case 3:
        if (input_index == inputs.length) {
          return [i, output, relative_base];
        }
        instrs[instrs[i + 1] | 0] = inputs[input_index];
        input_index++;
        i += 2;
        break;

      case 4:
        output.push(parameters[0]);
        i += 2;
        break;

      case 5:
        if (parameters[0] !== 0) {
          i = parameters[1];
        } else {
          i += 3;
        }
        break;

      case 6:
        if (parameters[0] === 0) {
          i = parameters[1];
        } else {
          i += 3;
        }
        break;

      case 7:
        if (parameters[0] < parameters[1]) {
          instrs[instrs[i + 3] | 0] = 1;
        } else {
          instrs[instrs[i + 3] | 0] = 0;
        }
        i += 4;
        break;

      case 8:
        if (parameters[0] === parameters[1]) {
          instrs[instrs[i + 3] | 0] = 1;
        } else {
          instrs[instrs[i + 3] | 0] = 0;
        }
        i += 4;
        break;

      case 9:
        relative_base += parameters[0];
        i += 2;
        break;

      default:
        throw `invalid command : ${cmd}`;
    }
  }
};

const f0 = (instrs) => {
  let res = run_instrs(instrs, [1]);
  return res[1];
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 0, file: "test_input0.txt" },
  { f: f0, expected: 0, file: "test_input1.txt" },
  { f: f0, expected: 0, file: "test_input2.txt" },
]);

benchmark(f0, f1);
