import { benchmark, test } from "./main.js";

// floor : 0
// free : 1
// occupied : 2

export const format = (lines) => {
  const get_identifier = (c) => {
    switch (c) {
      case "L":
        return 1;
      case "#":
        return 2;
      case ".":
        return 0;
      default:
        raise`wrong input : ${c}`;
    }
  };
  return lines.map((l) => Array.from(l).map(get_identifier));
};

const next_state0 = (x, y, grid) => {
  const [width, height] = [grid.length, grid[0].length];

  let count = 0;

  for (let i = -1; i < 2; i++) {
    for (let j = -1; j < 2; j++) {
      if (i == 0 && j == 0) {
        continue;
      }
      const [newx, newy] = [x + i, y + j];
      if (newx < 0 || newy < 0 || newx >= width || newy >= height) {
        continue;
      }

      if (grid[newx][newy] === 2) {
        count++;
      }
    }
  }

  if (grid[x][y] === 1 && count === 0) {
    return 2;
  } else if (grid[x][y] === 2 && count >= 4) {
    return 1;
  } else {
    return grid[x][y];
  }
};

let iterate = (data, next_state_function) => {
  const grid = data.grid;
  const [width, height] = [grid.length, grid[0].length];

  let changed_count = 0;
  let copy = structuredClone(grid);

  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) {
      if (grid[x][y] === 0) {
        continue;
      }

      let new_state = next_state_function(x, y, grid);
      if (new_state !== grid[x][y]) {
        changed_count++;
      }
      copy[x][y] = new_state;
    }
  }

  data.grid = copy;
  return changed_count;
};

const count_occupied_seats = (grid) => {
  return grid
    .flatMap((l) => l.map((seat) => seat === 2))
    .reduce((a, b) => a + b, 0);
};

const f0 = (input) => {
  const data = { grid: input };
  while (iterate(data, next_state0) > 0);
  return count_occupied_seats(data.grid);
};

const next_state1 = (x, y, grid) => {
  const [width, height] = [grid.length, grid[0].length];

  let count = 0;

  for (let iDir = -1; iDir < 2; iDir++) {
    for (let yDir = -1; yDir < 2; yDir++) {
      if (iDir == 0 && yDir == 0) {
        continue;
      }
      let steps = 1;
      while (true) {
        let [newx, newy] = [x + iDir * steps, y + yDir * steps];

        if (newx < 0 || newy < 0 || newx >= width || newy >= height) {
          break;
        }

        if (grid[newx][newy] === 2) {
          count++;
          break;
        } else if (grid[newx][newy] === 1) {
          break;
        }
        steps++;
      }
    }
  }

  if (grid[x][y] === 1 && count === 0) {
    return 2;
  } else if (grid[x][y] === 2 && count >= 5) {
    return 1;
  } else {
    return grid[x][y];
  }
};

const f1 = (input) => {
  const data = { grid: input };
  while (iterate(data, next_state1) > 0);
  return count_occupied_seats(data.grid);
};

test([
  { f: f0, expected: 37 },
  { f: f1, expected: 26 },
]);

benchmark(f0, f1);
