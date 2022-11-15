import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines[0].split(",").map((x) => parseInt(x));
};

const run_instrs0 = (instrs) => {
  let output = [];
  let i = 0;
  while (true) {
    let cmd = instrs[i];
    let parameters = [null, null, null];
    for (let j = 0; j < 3; j++) {
      let b = 0;
      if (cmd.toString().length > 2 + j) {
        b = (cmd % Math.pow(10, j + 3)) - (cmd % Math.pow(10, j + 2));
      }
      let val = instrs[i + 1 + j];
      if (b === 0) {
        parameters[j] = instrs[val];
      } else {
        parameters[j] = val;
      }
    }
    cmd = cmd % 100;
    switch (cmd) {
      case 1:
        instrs[instrs[i + 3]] = parameters[0] + parameters[1];
        i += 4;
        break;

      case 2:
        instrs[instrs[i + 3]] = parameters[0] * parameters[1];
        i += 4;
        break;

      case 99:
        return output;

      case 3:
        instrs[instrs[i + 1]] = 1;
        i += 2;
        break;

      case 4:
        output += parameters[0];
        i += 2;
        break;

      default:
        throw `invalid command : ${cmd}`;
    }
  }
};

const f0 = (input) => {
  return run_instrs0(input);
};

const run_instrs1 = (instrs) => {
  let output = [];
  let i = 0;
  while (true) {
    let cmd = instrs[i];
    let parameters = [null, null, null];
    for (let j = 0; j < 3; j++) {
      let b = 0;
      if (cmd.toString().length > 2 + j) {
        b = (cmd % Math.pow(10, j + 3)) - (cmd % Math.pow(10, j + 2));
      }
      let val = instrs[i + 1 + j];
      if (b === 0) {
        parameters[j] = instrs[val];
      } else {
        parameters[j] = val;
      }
    }
    cmd = cmd % 100;
    switch (cmd) {
      case 1:
        instrs[instrs[i + 3]] = parameters[0] + parameters[1];
        i += 4;
        break;

      case 2:
        instrs[instrs[i + 3]] = parameters[0] * parameters[1];
        i += 4;
        break;

      case 99:
        return output;

      case 3:
        instrs[instrs[i + 1]] = 5;
        i += 2;
        break;

      case 4:
        output += parameters[0];
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
          instrs[instrs[i + 3]] = 1;
        } else {
          instrs[instrs[i + 3]] = 0;
        }
        i += 4;
        break;

      case 8:
        if (parameters[0] === parameters[1]) {
          instrs[instrs[i + 3]] = 1;
        } else {
          instrs[instrs[i + 3]] = 0;
        }
        i += 4;
        break;

      default:
        throw `invalid command : ${cmd}`;
    }
  }
};

const f1 = (input) => {
  return run_instrs1(input);
};

// test([
//   { f: f0, expected: 0 },
//   { f: f1, expected: 0 },
// ]);

benchmark(f0, f1);
