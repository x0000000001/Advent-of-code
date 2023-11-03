import { match } from "assert";
import { benchmark, test } from "./main.js";

type InputType = {
  instrs: Instr[];
};

type Instr = {
  name: string;
  args: number[];
};

type OutputType = number;

export const format = (lines: string[]): InputType => {
  return {
    instrs: lines.map((l) => {
      let w = l.split(" ");
      let args = [];
      switch (w[0]) {
        case "addx":
          args.push(parseInt(w[1]));
          break;
        case "noop":
          break;
      }

      return { name: w[0], args };
    }),
  };
};

const f0 = ({ instrs }: InputType): OutputType => {
  let x = 1;
  let cycle = 1;
  let sum = 0;
  let interesting = [20, 60, 100, 140, 180, 220];

  for (const instr of instrs) {
    switch (instr.name) {
      case "noop":
        cycle += 1;
        if (interesting.includes(cycle)) {
          sum += cycle * x;
        }
        break;

      case "addx":
        cycle += 1;
        if (interesting.includes(cycle)) {
          sum += cycle * x;
        }
        cycle += 1;
        x += instr.args[0];
        if (interesting.includes(cycle)) {
          sum += cycle * x;
        }
        break;
    }
  }

  return sum;
};

const f1 = ({ instrs }: InputType): OutputType => {
  let grid = Array.from({ length: 6 }, () =>
    Array.from({ length: 40 }, () => false)
  );

  let current_instr = 0;
  let current_count = 0;
  let x = 0;

  for (let i = 0; i < 6; i++) {
    for (let j = 0; j < 40; j++) {
      switch (instrs[current_instr].name) {
        case "noop":
          current_instr++;
          break;
        case "addx":
          current_count++;
          if (current_count == 2) {
            current_count = 0;
            x += instrs[current_instr].args[0];
            current_instr++;
          }
          break;
      }

      if (Math.abs(j - x) < 2) {
        grid[i][j] = true;
      }
    }
  }

  for (let i = 0; i < 6; i++) {
    console.log(grid[i].reduce((acc, b) => acc + (b ? "#" : "."), ""));
  }

  return 0;
};

test([
  { f: f0, expected: 13140 },
  { f: f1, expected: 0 },
]);

benchmark(f0, f1);
