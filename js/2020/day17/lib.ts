import { copyFileSync } from "fs";
import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return lines.map((l) => Array.from(l).map((c) => c == "#"));
};

const iterate3 = (cubes: Set<number>): Set<number> => {
  let new_state = new Set<number>();
  const [minx, maxx, miny, maxy, minz, maxz] = Array.from(cubes).reduce(
    ([minx, maxx, miny, maxy, minz, maxz], key) => {
      let [x, y, z] = dehash3(key);
      return [
        Math.min(minx, x),
        Math.max(maxx, x),
        Math.min(miny, y),
        Math.max(maxy, y),
        Math.min(minz, z),
        Math.max(maxz, z),
      ];
    },
    [
      Number.MAX_SAFE_INTEGER,
      0,
      Number.MAX_SAFE_INTEGER,
      0,
      Number.MAX_SAFE_INTEGER,
      0,
    ]
  );

  // not efficient but js sets equality is not yet customizable
  const vals = Array.from(cubes.values());

  for (let x = minx - 1; x < maxx + 2; x++) {
    for (let y = miny - 1; y < maxy + 2; y++) {
      for (let z = minz - 1; z < maxz + 2; z++) {
        let count = 0;
        for (const newx of [x - 1, x, x + 1]) {
          for (const newy of [y - 1, y, y + 1]) {
            for (const newz of [z - 1, z, z + 1]) {
              if (newx === x && newy === y && newz === z) continue;
              if (cubes.has(hash3(newx, newy, newz))) {
                count++;
              }
            }
          }
        }
        if (cubes.has(hash3(x, y, z))) {
          if ([2, 3].includes(count)) new_state.add(hash3(x, y, z));
        } else {
          if ([3].includes(count)) new_state.add(hash3(x, y, z));
        }
      }
    }
  }

  return new_state;
};

const hash3 = (x: number, y: number, z: number): number => {
  return x * 1000000000000 + y * 1000000 + z;
};

const dehash3 = (key: number): [number, number, number] => {
  let x_factor = 1;
  let y_factor = 1;
  let z_factor = 1;
  let x = 0;
  let y = 0;
  let z = 0;
  if (500000 < key % 1000000) {
    z_factor = -1;
    key += 2 * (1000000 - (key % 1000000));
  }

  z = key % 1000000;
  key %= 1000000;

  if (500000 < key % 1000000) {
    y_factor = -1;
    key += 2 * (1000000 - (key % 1000000));
  }

  y = key % 1000000;
  key %= 1000000;

  if (500000 < key % 1000000) {
    x_factor = -1;
    key += 2 * (1000000 - (key % 1000000));
  }

  x = key % 1000000;
  key %= 1000000;

  return [x * x_factor, y * y_factor, z * z_factor];
};

const get_cubes3 = (input: boolean[][]): Set<number> => {
  let cubes = new Set<number>();

  for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input[0].length; j++) {
      if (input[i][j]) {
        cubes.add(hash3(i, j, 0));
      }
    }
  }

  return cubes;
};

const count_cubes3 = (cubes: Set<number>): number => {
  return cubes.size;
};

const f0 = (input) => {
  let cubes = get_cubes3(input);
  for (let _ = 0; _ < 6; _++) {
    cubes = iterate3(cubes);
  }
  return count_cubes3(cubes);
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 112 },
  { f: f1, expected: 848 },
]);

benchmark(f0, f1);
