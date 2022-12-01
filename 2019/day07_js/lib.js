import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines[0].split(",").map((x) => parseInt(x));
};

const run_instrs = (instrs, inputs, i = 0) => {
  let output = [];
  let input_index = 0;
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
        return [null, output];

      case 3:
        if (input_index == inputs.length) {
          return [i, output];
        }
        instrs[instrs[i + 1]] = inputs[input_index];
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

const iterate_seed = (data, index = 0) => {
  let seed = data.seed;
  if (index == seed.length - 1) {
    return false;
  }

  if (!iterate_seed(data, index + 1)) {
    let current_val = seed[index];
    let ordered = seed.slice(index, seed.length);
    ordered.sort((a, b) => a - b);
    let index_of_current = ordered.indexOf(current_val);
    if (index_of_current == ordered.length - 1) {
      return false;
    }
    let new_current = ordered.splice(index_of_current + 1, 1)[0];
    seed[index] = new_current;
    for (let i = 0; i < ordered.length; i++) {
      seed[index + i + 1] = ordered[i];
    }
    data.seed = seed;
    return true;
  } else {
    return true;
  }
};

const seed_score0 = (seed, instrs) => {
  let current = 0;
  for (let index = 0; index < 5; index++) {
    current = parseInt(
      run_instrs(structuredClone(instrs), [seed[index], current])[1]
    );
  }
  return current;
};

const f0 = (instrs) => {
  let seed = [0, 1, 2, 3, 4];
  let data = { seed };
  let [max, max_seed] = [0, null];
  while (true) {
    const score = seed_score0(data.seed, instrs);
    if (score > max) {
      max = score;
      max_seed = structuredClone(data.seed);
    }

    if (!iterate_seed(data)) {
      break;
    }
  }
  return max;
};

const seed_score1 = (seed, instrs) => {
  let current = [0];
  let stopped_count = 0;
  let instrs_pointers = new Array(5).fill(0);
  let machines = new Array(5).fill(0).map((_) => structuredClone(instrs));
  let current_machine = 0;
  let first_round = true;
  while (stopped_count != 5) {
    if (first_round) {
      current.unshift(seed[current_machine]);
    }

    if (machines[current_machine] === null) {
      current = [];
      current_machine = (current_machine + 1) % 5;
      continue;
    }

    let res = run_instrs(
      machines[current_machine],
      current,
      instrs_pointers[current_machine]
    );
    let new_pointer = res[0];
    instrs_pointers[current_machine] = new_pointer;
    current = res[1];

    if (new_pointer === null) {
      stopped_count++;
      machines[current_machine] === null;
    }
    current_machine = (current_machine + 1) % 5;

    if (current_machine === 0) {
      first_round = false;
    }
  }

  return current[0];
};

const f1 = (instrs) => {
  let seed = [5, 6, 7, 8, 9];
  let data = { seed };
  let [max, max_seed] = [0, null];
  while (true) {
    const score = seed_score1(data.seed, instrs);
    if (score > max) {
      max = score;
      max_seed = structuredClone(data.seed);
    }
    if (!iterate_seed(data)) {
      break;
    }
  }
  return max;
};

test([
  { f: f0, expected: 43210, file: "test_input1.txt" },
  { f: f0, expected: 54321, file: "test_input2.txt" },
  { f: f0, expected: 65210, file: "test_input3.txt" },
  { f: f1, expected: 139629729, file: "test_input4.txt" },
  { f: f1, expected: 18216, file: "test_input5.txt" },
]);

benchmark(f0, f1);
