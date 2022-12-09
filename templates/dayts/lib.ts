import { benchmark, test } from "./main.js";

type InputType = {
  lines: string[];
};

type OutputType = number;

export const format = (lines: string[]): InputType => {
  return { lines };
};

const f0 = ({ lines }: InputType): OutputType => {
  return 0;
};

const f1 = ({ lines }: InputType): OutputType => {
  return 0;
};

test([
  { f: f0, expected: 0 },
  { f: f1, expected: 0 },
]);

benchmark(f0, f1);
