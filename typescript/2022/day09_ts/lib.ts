import { match } from "assert";
import { benchmark, test } from "./main.js";

type InputType = {
  directions: [string, number][];
};

type OutputType = number;

type Pos = {
  x: number;
  y: number;
};

export const format = (lines: string[]): InputType => {
  const get_dir = (l: string): [string, number] => {
    let words = l.split(" ");
    return [words[0], parseInt(words[1])];
  };

  return { directions: lines.map(get_dir) };
};

const count_tail_positions = ({ directions }: InputType, n: number): number => {
  let rope: Pos[] = Array.from({ length: n }, () => {
    return { x: 0, y: 0 };
  });
  let visited: Set<number> = new Set();

  const signature = (p: Pos): number => p.x * 100000 + p.y;

  visited.add(signature(rope[n - 1]));

  directions.forEach(([dir, amplitude]) => {
    for (let i = 0; i < amplitude; i++) {
      switch (dir) {
        case "R":
          rope[0].y += 1;
          break;
        case "L":
          rope[0].y -= 1;
          break;
        case "U":
          rope[0].x -= 1;
          break;
        case "D":
          rope[0].x += 1;
          break;
        default:
          throw new Error("no");
      }

      for (let j = 1; j < n; j++) {
        if (
          Math.abs(rope[j].x - rope[j - 1].x) > 1 ||
          Math.abs(rope[j].y - rope[j - 1].y) > 1
        ) {
          if (rope[j - 1].x > rope[j].x) {
            rope[j].x += 1;
          } else if (rope[j - 1].x < rope[j].x) {
            rope[j].x -= 1;
          }

          if (rope[j - 1].y > rope[j].y) {
            rope[j].y += 1;
          } else if (rope[j - 1].y < rope[j].y) {
            rope[j].y -= 1;
          }
        }
      }
      visited.add(signature(rope[n - 1]));

      // console.log(head, tail);
    }
  });

  return visited.size;
};

const f0 = ({ directions }: InputType): OutputType => {
  return count_tail_positions({ directions }, 2);
};

const f1 = ({ directions }: InputType): OutputType => {
  return count_tail_positions({ directions }, 10);
};

test([
  { f: f0, expected: 13 },
  { f: f1, expected: 36, file: "test_input1.txt" },
]);

// 6066 too low
benchmark(f0, f1);
