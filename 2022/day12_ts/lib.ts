import { benchmark, test } from "./main.js";

type InputType = {
  map: number[][];
  endx: number;
  startx: number;
  endy: number;
  starty: number;
};

type OutputType = number;

export const format = (lines: string[]): InputType => {
  let startx, starty, endx, endy;
  let [h, w] = [lines.length, lines[0].length];
  let map = Array.from({ length: h }, () => Array.from({ length: w }, () => 0));

  for (let i = 0; i < h; i++) {
    let chars = lines[i].split("");
    for (let j = 0; j < w; j++) {
      switch (chars[j]) {
        case "S":
          startx = i;
          starty = j;
          map[i][j] = 0;
          break;
        case "E":
          endx = i;
          endy = j;
          map[i][j] = 25;
          break;
        default:
          map[i][j] = chars[j].charCodeAt(0) - 97;
          break;
      }
    }
  }
  return { map, startx, endx, starty, endy };
};

// Djisktra
const f0 = ({ map, startx, endx, starty, endy }: InputType): OutputType => {
  let queue = [[0, startx, starty]];
  let [h, w] = [map.length, map[0].length];
  let scores = Array.from({ length: h }, () =>
    Array.from({ length: w }, () => Number.MAX_SAFE_INTEGER)
  );
  scores[startx][starty] = 0;

  while (queue.length != 0) {
    queue.sort((a, b) => b[0] - a[0]);
    let [score, x, y] = queue.pop();

    if (x == endx && y == endy) {
      return score;
    }

    let candidates = [];
    let new_score = score + 1;
    let height = map[x][y];

    if (x > 0 && scores[x - 1][y] > new_score && map[x - 1][y] <= height + 1) {
      candidates.push([x - 1, y]);
    }
    if (y > 0 && scores[x][y - 1] > new_score && map[x][y - 1] <= height + 1) {
      candidates.push([x, y - 1]);
    }
    if (
      x < h - 1 &&
      scores[x + 1][y] > new_score &&
      map[x + 1][y] <= height + 1
    ) {
      candidates.push([x + 1, y]);
    }
    if (
      y < w - 1 &&
      scores[x][y + 1] > new_score &&
      map[x][y + 1] <= height + 1
    ) {
      candidates.push([x, y + 1]);
    }

    for (const [candidatex, candidatey] of candidates) {
      scores[candidatex][candidatey] = new_score;
      queue.push([new_score, candidatex, candidatey]);
    }
  }

  return -1;
};

// Complete reverse Djisktra
const f1 = ({ map, startx, endx, starty, endy }: InputType): OutputType => {
  let queue = [[0, endx, endy]];
  let [h, w] = [map.length, map[0].length];
  let scores = Array.from({ length: h }, () =>
    Array.from({ length: w }, () => Number.MAX_SAFE_INTEGER)
  );
  scores[endx][endy] = 0;

  while (queue.length != 0) {
    queue.sort((a, b) => b[0] - a[0]);
    let [score, x, y] = queue.pop();

    let candidates = [];
    let new_score = score + 1;
    let height = map[x][y];

    if (x > 0 && scores[x - 1][y] > new_score && map[x - 1][y] >= height - 1) {
      candidates.push([x - 1, y]);
    }
    if (y > 0 && scores[x][y - 1] > new_score && map[x][y - 1] >= height - 1) {
      candidates.push([x, y - 1]);
    }
    if (
      x < h - 1 &&
      scores[x + 1][y] > new_score &&
      map[x + 1][y] >= height - 1
    ) {
      candidates.push([x + 1, y]);
    }
    if (
      y < w - 1 &&
      scores[x][y + 1] > new_score &&
      map[x][y + 1] >= height - 1
    ) {
      candidates.push([x, y + 1]);
    }

    for (const [candidatex, candidatey] of candidates) {
      scores[candidatex][candidatey] = new_score;
      queue.push([new_score, candidatex, candidatey]);
    }
  }

  let min = Number.MAX_SAFE_INTEGER;

  for (let i = 0; i < h; i++) {
    for (let j = 0; j < w; j++) {
      if (map[i][j] == 0 && scores[i][j] < min) {
        min = scores[i][j];
      }
    }
  }

  return min;
};

test([
  { f: f0, expected: 31 },
  { f: f1, expected: 29 },
]);

benchmark(f0, f1);
