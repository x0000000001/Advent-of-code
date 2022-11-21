import { benchmark, test } from "./main.js";

type InputType = { [index: number]: bigint };

export const format = (lines: string[]) => {
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
const run_instrs = (state): void => {
  let instrs: bigint[] = state.instrs;
  let i: number = state.i;
  let outputs: bigint[] = state.outputs;
  let inputs: bigint[] = state.inputs;
  let relative_base: number = state.relative_base;

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
        state.i = null;
        state.relative_base = relative_base;
        return;

      case 3n:
        if (inputs.length === 0) {
          state.i = i;
          state.relative_base = relative_base;
          return;
        }
        instrs[parameters_as_pointers[0]] = inputs.shift();
        i += 2;
        break;

      case 4n:
        outputs.push(parameters[0]);
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

class Position {
  x: number;
  y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

const f0 = (instrs) => {
  let pos = new Position(0, 0);
  let visited = {};
  let facing = 0;
  let machine_state = {
    instrs,
    inputs: [],
    i: 0,
    outputs: [],
    relative_base: 0,
  };

  while (machine_state.i !== null) {
    machine_state.inputs.push(BigInt(visited[pos.x * 100000 + pos.y]) | 0n);

    run_instrs(machine_state);
    let paint_color: number = Number(machine_state.outputs.shift());
    let rotation: number = Number(machine_state.outputs.shift());

    visited[pos.x * 100000 + pos.y] = paint_color;
    facing = (facing + (rotation * 2 - 1) + 4) % 4;
    switch (facing) {
      case 0:
        pos.y++;
        break;
      case 1:
        pos.x++;
        break;
      case 2:
        pos.y--;
        break;
      case 3:
        pos.x--;
        break;
    }
  }

  return Object.keys(visited).length;
};

const f1 = (input) => {
  return 0;
};

test([
  // { f: f0, expected: 0 },
  // { f: f1, expected: 0 },
]);

benchmark(f0, f1);
