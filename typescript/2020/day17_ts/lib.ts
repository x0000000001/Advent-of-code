import { copyFileSync } from "fs";
import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return lines.map((l) => Array.from(l).map((c) => c == "#"));
};

const iterate3 = (cubes: Set<string>): Set<string> => {
  let new_state = new Set<string>();
  const [minx, maxx, miny, maxy, minz, maxz] = Array.from(cubes).reduce(
    ([minx, maxx, miny, maxy, minz, maxz], key) => {
      let [x, y, z] = dehash(key);
      // console.log(key, x, y, z);
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

  for (let x = minx - 1; x < maxx + 2; x++) {
    for (let y = miny - 1; y < maxy + 2; y++) {
      for (let z = minz - 1; z < maxz + 2; z++) {
        let count = 0;
        for (const newx of [x - 1, x, x + 1]) {
          for (const newy of [y - 1, y, y + 1]) {
            for (const newz of [z - 1, z, z + 1]) {
              if (newx === x && newy === y && newz === z) continue;
              if (cubes.has(hash([newx, newy, newz]))) {
                count++;
              }
            }
          }
        }
        if (cubes.has(hash([x, y, z]))) {
          if ([2, 3].includes(count)) new_state.add(hash([x, y, z]));
        } else {
          if ([3].includes(count)) new_state.add(hash([x, y, z]));
        }
      }
    }
  }

  return new_state;
};

const hash = (coordinates: number[]): string => {
  return coordinates.map((x) => x.toString()).join(",");
};

const dehash = (key: string): number[] => {
  return key.split(",").map((x) => parseInt(x));
};

const get_cubes = (input: boolean[][], n: number): Set<string> => {
  let cubes = new Set<string>();

  for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input[0].length; j++) {
      if (input[i][j]) {
        let l = new Array(n).fill(0);
        l[0] = i;
        l[1] = j;
        cubes.add(hash(l));
      }
    }
  }

  return cubes;
};

const count_cubes = (cubes: Set<string>): number => {
  return cubes.size;
};

const f0 = (input) => {
  let cubes = get_cubes(input, 3);
  for (let _ = 0; _ < 6; _++) {
    cubes = iterate3(cubes);
  }
  return count_cubes(cubes);
};

const iterate4 = (cubes: Set<string>): Set<string> => {
  let new_state = new Set<string>();
  const [minx, maxx, miny, maxy, minz, maxz, mink, maxk] = Array.from(
    cubes
  ).reduce(
    ([minx, maxx, miny, maxy, minz, maxz, mink, maxk], key) => {
      let [x, y, z, k] = dehash(key);
      // console.log(key, x, y, z, k);
      return [
        Math.min(minx, x),
        Math.max(maxx, x),
        Math.min(miny, y),
        Math.max(maxy, y),
        Math.min(minz, z),
        Math.max(maxz, z),
        Math.min(mink, k),
        Math.max(maxk, k),
      ];
    },
    [
      Number.MAX_SAFE_INTEGER,
      0,
      Number.MAX_SAFE_INTEGER,
      0,
      Number.MAX_SAFE_INTEGER,
      0,
      Number.MAX_SAFE_INTEGER,
      0,
    ]
  );

  for (let x = minx - 1; x < maxx + 2; x++) {
    for (let y = miny - 1; y < maxy + 2; y++) {
      for (let z = minz - 1; z < maxz + 2; z++) {
        for (let k = mink - 1; k < maxk + 2; k++) {
          let count = 0;
          for (const newx of [x - 1, x, x + 1]) {
            for (const newy of [y - 1, y, y + 1]) {
              for (const newz of [z - 1, z, z + 1]) {
                for (const newk of [k - 1, k, k + 1]) {
                  if (newx === x && newy === y && newz === z && newk === k)
                    continue;
                  if (cubes.has(hash([newx, newy, newz, newk]))) {
                    count++;
                  }
                }
              }
            }
          }
          if (cubes.has(hash([x, y, z, k]))) {
            if ([2, 3].includes(count)) new_state.add(hash([x, y, z, k]));
          } else {
            if ([3].includes(count)) new_state.add(hash([x, y, z, k]));
          }
        }
      }
    }
  }

  return new_state;
};

const f1 = (input) => {
  let cubes = get_cubes(input, 4);
  for (let _ = 0; _ < 6; _++) {
    cubes = iterate4(cubes);
  }
  return count_cubes(cubes);
};

test([
  { f: f0, expected: 112 },
  { f: f1, expected: 848 },
]);

benchmark(f0, f1);
