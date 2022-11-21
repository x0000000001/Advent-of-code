import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return parseInt(lines[0]);
};

const power_level = (x, y, serial) => {
  x++;
  y++;
  const rackid = x + 10;
  let p = rackid * y + serial;
  p *= rackid;
  let s = p.toString();
  return (s.length >= 3 ? parseInt(s.charAt(s.length - 3)) : 0) - 5;
};

const get_grid = (serial): number[][] => {
  let grid = Array.from({ length: 300 }, () => new Array(300).fill(0));

  for (let i = 0; i < 300; i++) {
    for (let j = 0; j < 300; j++) {
      grid[i][j] = power_level(i, j, serial);
    }
  }

  return grid;
};

const max_nxn_square = (grid: number[][], n): [number, number, number] => {
  let width = grid.length;
  let [xmax, ymax] = [0, 0];
  let max_score = 0;

  let column_sums = Array.from({ length: width - n }, () =>
    new Array(width).fill(0)
  );

  for (let j = 0; j < width; j++) {
    let subsum = 0;
    for (let k = 0; k < n; k++) {
      subsum += grid[k][j];
    }
    for (let i = 0; i < width - n; i++) {
      column_sums[i][j] = subsum;
      subsum += grid[i + n][j] - grid[i][j];
    }

    column_sums[width - n - 1][j] = subsum;
  }

  for (let i = 0; i < width - n; i++) {
    let subsum = 0;
    for (let k = 0; k < n; k++) {
      subsum += column_sums[i][k];
    }

    for (let j = 0; j < width - n; j++) {
      if (subsum > max_score) [xmax, ymax, max_score] = [i, j, subsum];
      subsum += column_sums[i][j + n] - column_sums[i][j];
    }

    if (subsum > max_score)
      [xmax, ymax, max_score] = [i, width - n - 1, subsum];
  }

  return [xmax + 1, ymax + 1, max_score];
};

const f0 = (input) => {
  return max_nxn_square(get_grid(input), 3).slice(0, 2);
};

const max_square = (grid: number[][]): [number, number, number] => {
  let [xmax, ymax, length_max] = [0, 0, 0];
  let max_score = 0;

  for (let i = 1; i < grid.length; i++) {
    const [x, y, score] = max_nxn_square(grid, i);
    if (score > max_score) {
      max_score = score;
      [xmax, ymax, length_max] = [x, y, i];
    }
  }

  return [xmax, ymax, length_max];
};

const f1 = (input) => {
  return max_square(get_grid(input));
};

test([
  { f: (x) => power_level(2, 4, x), expected: 4 },
  {
    f: (x) => f1(x).toString(),
    expected: "90,269,16",
    file: "test_input1.txt",
  },
  {
    f: (x) => f1(x).toString(),
    expected: "232,251,12",
    file: "test_input2.txt",
  },
]);

benchmark(f0, f1);
