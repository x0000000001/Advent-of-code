import { benchmark, test } from "./main.js";

type InputType = [[number, number, number, number]];

export const format = (lines: string[]) => {
  const compute_line = (line): [number, number, number, number] => {
    let words = line.split(" velocity=<");
    const [vx, vy] = words[1]
      .substring(0, words[1].length - 1)
      .split(",")
      .map((x) => parseInt(x));

    let temp0 = words[0].split("<");
    const [x, y] = temp0[1]
      .substring(0, temp0[1].length - 1)
      .split(",")
      .map((x) => parseInt(x));

    return [x, y, vx, vy];
  };
  return lines.map(compute_line);
};

const iterate = (input) => {
  for (let i = 0; i < input.length; i++) {
    input[i][0] += input[i][2];
    input[i][1] += input[i][3];
  }
};

const print_stars = (input, step) => {
  let [minx, miny, maxx, maxy] = input
    .map(([x, y, vx, vy]) => [x, y])
    .reduce(
      ([minx, miny, maxx, maxy], [x, y]) => [
        Math.min(x, minx),
        Math.min(y, miny),
        Math.max(x, maxx),
        Math.max(y, maxy),
      ],
      [Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER, 0, 0]
    );

  const [width, height] = [maxx - minx + 1, maxy - miny + 1];

  if (width > 200 || height > 200) {
    return;
  }

  let grid = Array.from(Array(width), () => new Array(height).fill(-1));

  for (let i = 0; i < input.length; i++) {
    grid[input[i][0] - minx][input[i][1] - miny] = i;
  }

  for (let j = 0; j < height; j++) {
    let line: string = "";
    for (let i = 0; i < width; i++) {
      line += grid[i][j] == -1 ? ". " : "# ";
    }
    console.log(line);
  }

  console.log(`TIME : ${step + 1} `);
};

const print_animation = (input, time_limit) => {
  let i = 0;
  while (i < time_limit) {
    iterate(input);
    print_stars(input, i);
    i++;
  }

  console.log("FINISHED PRINTING");
};

const f0 = (input: InputType) => {
  print_animation(input, 50000);
  return 0;
};

const f1 = (input) => {
  print_animation(input, 50000);
  return 0;
};

test([{ f: (x) => print_animation(x, 3), expected: 0 }]);

benchmark(f0, f1);
