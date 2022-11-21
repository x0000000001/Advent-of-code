import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((l) => {
    let words = l.split(" ");
    return [words[0], parseInt(words[1])];
  });
};

const run_cmds = (cmds) => {
  let acc = 0;
  let position = 0;
  let has_been_there = Array(cmds.length).fill(false);

  while (true) {
    if (has_been_there[position]) {
      return [false, acc];
    } else if (position == cmds.length) {
      return [true, acc];
    }

    has_been_there[position] = true;
    let [instr, n] = cmds[position];

    switch (instr) {
      case "nop":
        position++;
        break;
      case "acc":
        acc += n;
        position++;
        break;
      case "jmp":
        position += n;
        break;
    }
  }
};

const f0 = (cmds) => {
  return run_cmds(cmds)[1];
};

const f1 = (cmds) => {
  for (let i = 0; i < cmds.length; i++) {
    let cmd = cmds[i][0];
    let new_cmds = undefined;
    switch (cmd) {
      case "jmp":
        new_cmds = structuredClone(cmds);
        new_cmds[i][0] = "nop";
        break;
      case "nop":
        new_cmds = structuredClone(cmds);
        new_cmds[i][0] = "jmp";
        break;
      default:
        continue;
    }

    let [is_finished, acc] = run_cmds(new_cmds);
    if (is_finished) {
      return acc;
    }
  }
  return NaN;
};

test([
  { f: f0, expected: 5 },
  { f: f1, expected: 8 },
]);

benchmark(f0, f1);
