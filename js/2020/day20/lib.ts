import { benchmark, test } from "./main.js";

// R = rotated, NR = not flipped
// F = flipped, NF = not flipped
type border = 0 | 1 | 2 | 3;
type grid = boolean[][];
type orientation = "NRNF" | "NRF" | "RNF" | "RF";
type tile = { id: number; NRNF: grid; NRF: grid; RNF: grid; RF: grid };
type drawing = {
  free_tiles: tile[];
  used: { [key: number]: [number, number, grid, border[]] }; // (x,y,grid,free borders)
  used_ids: Map<number, number>; // (position hash: id) dictionnary
};

const flip_grid = (g: grid): grid => {
  return Array.from({ length: g.length }, (_, i) =>
    Array.from({ length: g[0].length }, (_, j) => g[g.length - i - 1][j])
  );
};

const hash_pos = ([x, y]) => {
  return x * 1000000 + y;
};

const rotate_grid = (g: grid): grid => {
  return Array.from({ length: g.length }, (_, i) =>
    Array.from(
      { length: g[0].length },
      (_, j) => g[g.length - i - 1][g[0].length - j - 1]
    )
  );
};

export const format = (lines: string[]): tile[] => {
  let current_grid: grid = [];
  let current_id = null;
  let tiles: tile[] = [];

  lines.forEach((l) => {
    if (l.startsWith("Tile")) {
      if (current_grid.length !== 0) {
        tiles.push({
          id: current_id,
          NRNF: current_grid,
          NRF: flip_grid(current_grid),
          RNF: rotate_grid(current_grid),
          RF: rotate_grid(flip_grid(current_grid)),
        });
      }
      current_grid = [];
      current_id = parseInt(l.split(" ")[1].replace(":", ""));
    } else {
      current_grid.push(Array.from(l).map((c) => c === "#"));
    }
  });

  return tiles;
};

// returns positions of second relative to first
// 0: top, 1: right, 2: bottom, 3: left
const grids_alignements = (g0: grid, g1: grid): border[] => {
  let rots = [];
  const [W, H] = [g0.length, g0[0].length];

  if (
    Array.from({ length: W }, (_, i) => g0[0][i] == g1[W - 1][i]).reduce(
      (a, b) => a && b,
      true
    )
  ) {
    rots.push(0);
  }

  if (
    Array.from({ length: W }, (_, i) => g0[W - 1][i] == g1[0][i]).reduce(
      (a, b) => a && b,
      true
    )
  ) {
    rots.push(2);
  }

  if (
    Array.from({ length: H }, (_, i) => g0[i][H - 1] == g1[i][0]).reduce(
      (a, b) => a && b,
      true
    )
  ) {
    rots.push(1);
  }

  if (
    Array.from({ length: H }, (_, i) => g0[i][0] == g1[i][H - 1]).reduce(
      (a, b) => a && b,
      true
    )
  ) {
    rots.push(3);
  }

  return rots;
};

const can_be_inserted = (x, y, g1: grid, d: drawing): boolean => {
  const positions_to_check: [[number, number], border][] = [
    [[x, y + 1], 0],
    [[x + 1, y], 1],
    [[x, y - 1], 2],
    [[x - 1, y], 3],
  ];

  return positions_to_check
    .map(([[newx, newy], border]) => {
      let h = hash_pos([newx, newy]);
      return (
        !d.used_ids.has(h) ||
        grids_alignements(d.used[d.used_ids.get(h)][2], g1).includes(border)
      );
    })
    .reduce((a, b) => a && b, true);
};

const free_borders = (x, y, d: drawing): border[] => {
  const positions_to_check: [[number, number], border][] = [
    [[x, y + 1], 0],
    [[x + 1, y], 1],
    [[x, y - 1], 2],
    [[x - 1, y], 3],
  ];

  return positions_to_check
    .filter(([[newx, newy], _]) => !d.used_ids.has(hash_pos([newx, newy])))
    .map(([, border]) => border);
};

const assemble_drawing = (d: drawing): boolean => {
  if (d.free_tiles.length === 0) {
    return true;
  }

  const new_tile = d.free_tiles.pop();

  const possible_positions: [number, number, grid, border[]][] = [
    new_tile.NRNF,
    new_tile.NRF,
    new_tile.RNF,
    new_tile.RF,
  ].flatMap((g1) =>
    Array.from(Object.values(d.used)).flatMap(([x, y, _, free_b]) =>
      free_b
        .map((border) => {
          switch (border) {
            case 0:
              return [x, y + 1];
            case 1:
              return [x + 1, y];
            case 2:
              return [x, y - 1];
            case 3:
              return [x - 1, y];
          }
        })
        .filter(([newx, newy]) => can_be_inserted(newx, newy, g1, d))
        .map(([newx, newy]) => [newx, newy, g1, free_borders(newx, newy, d)])
    )
  );
};

const f0 = (tiles) => {
  let d: drawing = {
    used: {},
    free_tiles: tiles,
    used_ids: new Map<number, number>(),
  };
  return 0;
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 20899048083289 },
  { f: f1, expected: 0 },
]);

benchmark(f0, f1);
