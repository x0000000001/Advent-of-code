import { benchmark, test } from "./main.js";

class Point {
  x: number;
  y: number;

  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }

  wayTo(other: Point) {
    return new Point(other.x - this.x, other.y - this.y);
  }

  get moduleSquared() {
    return this.x * this.x + this.y * this.y;
  }

  equals(other: Point) {
    return this.x === other.x && this.y === other.y;
  }
}

export const format = (lines: string[]) => {
  return lines.map((line) => Array.from(line).map((c) => c == "#"));
};

const is_point_between = (
  p0: Point,
  p1: Point,
  potential_middle: Point
): boolean => {
  let dist0 = p0.wayTo(p1);
  let dist1 = p0.wayTo(potential_middle);
  return (
    dist0.moduleSquared > dist1.moduleSquared &&
    dist0.x * dist1.x >= 0 &&
    dist0.y * dist1.y >= 0 &&
    dist0.x * dist1.y === dist1.x * dist0.y
  );
};

const get_points = (grid): Point[] => {
  let points = [];

  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      if (grid[i][j]) points.push(new Point(i, j));
    }
  }

  return points;
};

const get_best_score = (points: Point[]): [number, number] => {
  const scores = new Array(points.length).fill(0);

  for (let i = 0; i < points.length; i++) {
    let score: number = 0;
    const p0 = points[i];
    for (let j = 0; j < points.length; j++) {
      if (i == j) continue;
      let is_visible = true;
      const p1 = points[j];
      for (let k = 0; k < points.length; k++) {
        if (k == j || k == i) continue;
        if (is_point_between(p0, p1, points[k])) {
          is_visible = false;
          break;
        }
      }
      score += is_visible ? 1 : 0;
    }
    scores[i] = score;
  }

  return scores.reduce(
    ([i_max, max], val, i) => (val > max ? [i, val] : [i_max, max]),
    [-1, 0]
  );
};

const f0 = (grid) => {
  const points = get_points(grid);
  return get_best_score(points)[1];
};

const get_asteroid_score = (p: Point) => {
  return p.y * 100 + p.x;
};

const angle = (p0: Point, p1: Point) =>
  Math.atan2(p0.y - p1.y, p0.x - p1.x) * (180 / Math.PI);

const sort_points_by_rotation = (
  p0: Point,
  p1: Point,
  station: Point
): number => {
  if (p0.equals(p1)) {
    return 0;
  } else if (
    // aligned case
    are_aligned(p0, p1, station)
  ) {
    return p0.moduleSquared > p1.moduleSquared ? -1 : 1;
  } else {
    return angle(p0, station) < angle(p1, station) ? 1 : -1;
  }
};

const are_aligned = (p0: Point, p1: Point, reference: Point): boolean => {
  let diff0 = reference.wayTo(p0);
  let diff1 = reference.wayTo(p1);
  return (
    diff0.x * diff1.x >= 0 &&
    diff0.y * diff1.y >= 0 &&
    diff0.x * diff1.y === diff1.x * diff0.y
  );
};

const f1 = (grid) => {
  const points = get_points(grid);
  const station: Point = points.splice(get_best_score(points)[0], 1)[0];
  points.sort((p0, p1) => sort_points_by_rotation(p0, p1, station));
  let alignements = [];
  let current_align = [points[0]];

  for (let i = 1; i < points.length; i++) {
    if (are_aligned(points[i], points[i - 1], station)) {
      current_align.push(points[i]);
    } else {
      alignements.push(current_align);
      current_align = [points[i]];
    }
  }

  alignements.push(current_align);

  let removal_index = 0;
  let layer_index = 0;
  let alignements_index = 0;

  while (removal_index < 200) {
    if (alignements[alignements_index].length > layer_index) {
      console.log(
        removal_index,
        alignements[alignements_index][layer_index].y,
        alignements[alignements_index][layer_index].x
      );
      removal_index++;
    }

    if (alignements_index === alignements.length - 1) {
      alignements_index = 0;
      layer_index++;
    } else {
      alignements_index++;
    }
  }

  return get_asteroid_score(alignements[alignements_index - 1][layer_index]);
};

test([
  { f: f0, expected: 210 },
  { f: f0, expected: 8, file: "test_input0.txt" },
  { f: f0, expected: 33, file: "test_input1.txt" },
  { f: f0, expected: 35, file: "test_input2.txt" },
  // { f: f1, expected: undefined, file: "test_input0.txt" },
  { f: f1, expected: 802 },
]);

benchmark(f0, f1);
