import { type } from "os";
import { benchmark, test } from "./main.js";

type Position = [number, number];
type Interval = [number, number];

type InputType = {
  sensors: [Position, Position][];
};

type OutputType = number;

export const format = (lines: string[]): InputType => {
  const read_coords = (s0: string, s1: string): Position => {
    return [
      parseInt(s0.substring(2, s0.length - 1)),
      parseInt(s1.substring(2)),
    ];
  };

  return {
    sensors: lines.map((l) => {
      let words = l.split(" ");
      return [read_coords(words[2], words[3]), read_coords(words[8], words[9])];
    }),
  };
};

const manhattan_distance = (p0: Position, p1: Position): number => {
  return Math.abs(p0[0] - p1[0]) + Math.abs(p0[1] - p1[1]);
};

const interval_intersection = (int0: Interval, int1: Interval): Interval => {
  if (int0[0] > int1[0]) {
    [int0, int1] = [int1, int0];
  }

  if (int0[1] < int1[0]) {
    return null;
  } else {
    return [int1[0], Math.min(int1[1], int0[1])];
  }
};

const count_total_inter = (intervals: Interval[]): number => {
  let count = 0;
  for (let i = 0; i < intervals.length; i++) {
    let inters = [];
    for (let j = i + 1; j < intervals.length; j++) {
      let interj = interval_intersection(intervals[i], intervals[j]);
      if (interj !== null) {
        inters.push(interj);
      }
    }

    count += intervals[i][1] - intervals[i][0] + 1;
    count -= count_total_inter(inters);
  }
  return count;
};

const hash = (p: Position): number => {
  return p[0] * 4000000 + p[1];
};

const hash_to_pos = (h: number): Position => {
  return [Math.floor(h / 4000000), h % 4000000];
};

const count_impossible_positions = (
  { sensors }: InputType,
  row: number
): number => {
  let intervals: Interval[] = [];

  for (const [sensor, beacon] of sensors) {
    let distance = manhattan_distance(sensor, beacon);
    let y_distance = Math.abs(row - sensor[1]);
    let width_on_line = distance - y_distance;

    if (width_on_line >= 0) {
      intervals.push([sensor[0] - width_on_line, sensor[0] + width_on_line]);
    }
  }

  let count = count_total_inter(intervals);

  let beacon_set: Set<number> = new Set();
  sensors.forEach(([_, b]) => beacon_set.add(hash(b)));

  for (const b_hash of beacon_set) {
    if (hash_to_pos(b_hash)[1] == row) {
      count--;
    }
  }

  return count;
};

const f0 = ({ sensors }: InputType): OutputType => {
  return count_impossible_positions({ sensors }, 2000000);
};

type Square = [Position, Position, Position, Position];

const diamond_contains_square = (
  sensor: Position,
  amplitude: number,
  square: Square
): boolean => {
  return (
    manhattan_distance(square[0], sensor) <= amplitude &&
    manhattan_distance(square[1], sensor) <= amplitude &&
    manhattan_distance(square[2], sensor) <= amplitude &&
    manhattan_distance(square[3], sensor) <= amplitude
  );
};

const square_is_point = (sq: Square): boolean => {
  return (
    sq[0][0] === sq[1][0] &&
    sq[1][0] === sq[2][0] &&
    sq[2][0] === sq[3][0] &&
    sq[0][1] === sq[1][1] &&
    sq[1][1] === sq[2][1] &&
    sq[2][1] === sq[3][1]
  );
};

const squares_are_equal = (sq0: Square, sq1: Square): boolean => {
  for (let i = 0; i < 4; i++) {
    for (let j = 0; j < 2; j++) {
      if (sq0[i][j] !== sq1[i][j]) {
        return false;
      }
    }
  }

  return true;
};

// divide and conquer
const divide_square = (sq: Square): Square[] => {
  let middle: Position = [
    Math.floor((sq[0][0] + sq[1][0]) / 2),
    Math.floor((sq[0][1] + sq[2][1]) / 2),
  ];

  let res: Square[] = [
    [sq[0], [middle[0], sq[0][1]], [sq[0][0], middle[1]], middle],
    [
      [middle[0] + 1, sq[0][1]],
      sq[1],
      [middle[0] + 1, middle[1]],
      [sq[1][0], middle[1]],
    ],
    [
      [sq[0][0], middle[1] + 1],
      [middle[0], middle[1] + 1],
      sq[2],
      [middle[0], sq[2][1]],
    ],
    [
      [middle[0] + 1, middle[1] + 1],
      [sq[1][0], middle[1] + 1],
      [middle[0] + 1, sq[2][1]],
      sq[3],
    ],
  ];
  res = res.filter((sqp) => !squares_are_equal(sqp, sq));
  return res;
};

//
const find_signal_hash = ({ sensors }: InputType, width: number): number => {
  let amplitudes = sensors.map(([s, b]) => manhattan_distance(s, b));

  let queue: Square[] = [
    [
      [0, 0],
      [width, 0],
      [0, width],
      [width, width],
    ],
  ];

  while (queue.length !== 0) {
    let square = queue.pop();
    let is_contained = sensors
      .map(([s, _], i) => diamond_contains_square(s, amplitudes[i], square))
      .reduce((acc, b) => acc || b, false);

    if (is_contained) {
      continue;
    }

    if (square_is_point(square)) {
      return hash(square[0]);
    }

    for (const m of divide_square(square)) {
      queue.push(m);
    }
  }

  return 0;
};

const f1 = ({ sensors }: InputType): OutputType => {
  return find_signal_hash({ sensors }, 4000000);
};

test([
  {
    f: (input) => count_impossible_positions(input, 10),
    expected: 26,
  },
  { f: (input) => find_signal_hash(input, 20), expected: 56000011 },
]);

benchmark(f0, f1);
