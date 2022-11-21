import { benchmark, test } from "./main.js";

type InputType = { [index: number]: bigint };

export const format = (lines: string[]): InputType => {
  let instrs: InputType = {};
  lines[0].split(",").forEach((c, i) => (instrs[i] = BigInt(c)));
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
  inputs: bigint[],
  i: number = 0,
  relative_base: number = 0
): [number, bigint[], number] => {
  let output = [];
  let input_index = 0;
  while (true) {
    let cmd: bigint = instrs[i] === undefined ? 0n : instrs[i];
    let cmd_string: string = cmd.toString();
    let parameters: bigint[] = [null, null, null];
    let parameters_as_pointers: number[] = [null, null, null];
    for (let j = 0; j < 3; j++) {
      let mode: number = 0;
      if (cmd_string.length > 2 + j) {
        mode = parseInt(cmd_string.charAt(cmd_string.length - (3 + j)));
      }
      let val_as_pointer: number =
        instrs[i + 1 + j] === undefined ? 0 : Number(instrs[i + 1 + j]);
      switch (mode) {
        case 0:
          parameters[j] =
            instrs[val_as_pointer] === undefined ? 0n : instrs[val_as_pointer];
          parameters_as_pointers[j] = val_as_pointer;
          break;
        case 1:
          parameters[j] = instrs[i + 1 + j];
          break;
        case 2:
          parameters[j] =
            instrs[relative_base + val_as_pointer] === undefined
              ? 0n
              : instrs[relative_base + val_as_pointer];
          parameters_as_pointers[j] = val_as_pointer + relative_base;
          break;
      }
    }
    cmd %= 100n;

    switch (cmd) {
      case 1n:
        instrs[parameters_as_pointers[2]] = parameters[0] + parameters[1];
        i += 4;
        break;

      case 2n:
        instrs[parameters_as_pointers[2]] = parameters[0] * parameters[1];
        i += 4;
        break;

      case 99n:
        return [null, output, relative_base];

      case 3n:
        if (input_index == inputs.length) {
          return [i, output, relative_base];
        }
        instrs[parameters_as_pointers[0]] = inputs[input_index];
        input_index++;
        i += 2;
        break;

      case 4n:
        output.push(parameters[0]);
        i += 2;
        break;

      case 5n:
        if (parameters[0] !== 0n) {
          i = Number(parameters[1]);
        } else {
          i += 3;
        }
        break;

      case 6n:
        if (parameters[0] === 0n) {
          i = Number(parameters[1]);
        } else {
          i += 3;
        }
        break;

      case 7n:
        if (parameters[0] < parameters[1]) {
          instrs[parameters_as_pointers[2]] = 1n;
        } else {
          instrs[parameters_as_pointers[2]] = 0n;
        }
        i += 4;
        break;

      case 8n:
        if (parameters[0] === parameters[1]) {
          instrs[parameters_as_pointers[2]] = 1n;
        } else {
          instrs[parameters_as_pointers[2]] = 0n;
        }
        i += 4;
        break;

      case 9n:
        relative_base += Number(parameters[0]);
        i += 2;
        break;

      default:
        throw `invalid command : ${cmd}`;
    }
  }
};

const f0 = (instrs) => {
  let res = run_instrs(instrs, [1n]);
  if (res[0] !== null) {
    console.log("programm didn't finish");
  }
  return res[1];
};

const f1 = (instrs) => {
  let res = run_instrs(instrs, [2n]);
  if (res[0] !== null) {
    console.log("programm didn't finish");
  }
  return res[1];
};

test([
  { f: f0, expected: 0, file: "test_input0.txt" },
  { f: f0, expected: 0, file: "test_input1.txt" },
  { f: f0, expected: 0, file: "test_input2.txt" },
]);

benchmark(f0, f1);
