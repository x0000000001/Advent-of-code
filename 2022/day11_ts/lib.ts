import { benchmark, test } from "./main.js";

type InputType = {
  monkeys: Monkey[];
};

type Monkey = {
  n: number;
  items: number[];
  op: (n: number) => number;
  test: number;
  monkey_true: number;
  monkey_false: number;
};

type OutputType = number;

export const format = (lines: string[]): InputType => {
  let monkeys = [];
  let i = 0;

  while (i < lines.length) {
    let n = parseInt(lines[i].substring(0, lines[i].length - 1).split(" ")[1]);
    let items = lines[i + 1]
      .replace(",", "")
      .split(" ")
      .slice(4)
      .map((s) => parseInt(s));
    let w = lines[i + 2].split(" ");
    let val = parseInt(w[w.length - 1]);
    let op;

    if (w[w.length - 1] == "old") {
      op = (n) => n * n;
    } else {
      switch (w[w.length - 2]) {
        case "+":
          op = (n) => n + val;
          break;
        case "*":
          op = (n) => n * val;
          break;

        default:
          throw new Error("unknown operation");
      }
    }

    let test = parseInt(lines[i + 3].split(" ")[5]);
    let monkey_true = parseInt(lines[i + 4].split(" ")[9]);
    let monkey_false = parseInt(lines[i + 5].split(" ")[9]);

    monkeys.push({
      n,
      items,
      op,
      test,
      monkey_true,
      monkey_false,
    });

    i += 6;
  }

  return {
    monkeys,
  };
};

const f0 = ({ monkeys }: InputType): OutputType => {
  let inspections = Array.from({ length: monkeys.length }, () => 0);

  for (let _ = 0; _ < 20; _++) {
    for (let j = 0; j < monkeys.length; j++) {
      inspections[j] += monkeys[j].items.length;
      monkeys[j].items.reverse();
      while (monkeys[j].items.length > 0) {
        let worry = monkeys[j].items.pop();
        worry = Math.floor(monkeys[j].op(worry) / 3);
        if (worry % monkeys[j].test == 0) {
          monkeys[monkeys[j].monkey_true].items.push(worry);
        } else {
          monkeys[monkeys[j].monkey_false].items.push(worry);
        }
      }
    }
  }

  inspections.sort((x, y) => x - y);

  return (
    inspections[inspections.length - 1] * inspections[inspections.length - 2]
  );
};

const ppcm = (x, y): number => {
  return (x * y) / pgcd(x, y);
};

const pgcd = (x, y): number => {
  if (x > y) {
    let temp = x;
    x = y;
    y = temp;
  }

  if (y % x == 0) {
    return x;
  } else {
    return pgcd(y % x, x);
  }
};

const f1 = ({ monkeys }: InputType): OutputType => {
  let inspections = Array.from({ length: monkeys.length }, () => 0);
  let lcm = monkeys.map((m) => m.test).reduce((acc, x) => ppcm(acc, x), 1);

  for (let _ = 0; _ < 10000; _++) {
    for (let j = 0; j < monkeys.length; j++) {
      inspections[j] += monkeys[j].items.length;
      monkeys[j].items.reverse();
      while (monkeys[j].items.length > 0) {
        let worry = monkeys[j].items.pop();
        worry = monkeys[j].op(worry) % lcm;
        if (worry % monkeys[j].test == 0) {
          monkeys[monkeys[j].monkey_true].items.push(worry);
        } else {
          monkeys[monkeys[j].monkey_false].items.push(worry);
        }
      }
    }
  }

  inspections.sort((x, y) => x - y);

  return (
    inspections[inspections.length - 1] * inspections[inspections.length - 2]
  );
};

test([
  { f: f0, expected: 10605 },
  { f: f1, expected: 2713310158 },
]);

benchmark(f0, f1);
