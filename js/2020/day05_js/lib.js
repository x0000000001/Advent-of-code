import { benchmark, test } from "./main.js";

export const format = (lines) => {
  let bool_list = (s) => {
    return Array.from(s).map((c) => ["B", "R"].includes(c));
  };
  return lines.map((l) => {
    return [bool_list(l.substring(0, 7)), bool_list(l.substring(7))];
  });
};

const bin_to_dec = (bits) => {
  return bits.reduce((acc, b) => acc * 2 + b, 0);
};

const seat_id = ([row, column]) => {
  return bin_to_dec(row) * 8 + bin_to_dec(column);
};

const f0 = (input) => {
  return Math.max(...input.map(seat_id));
};

const f1 = (input) => {
  let ids = input.map(seat_id).sort((a, b) => a - b);
  let array_index = 0;
  let missing_count = 0;
  for (let index = 0; index < 1020; index++) {
    if (ids[array_index] === index) {
      if (missing_count === 1) {
        return index - 1;
      }
      array_index += 1;
      missing_count = 0;
    } else {
      missing_count++;
    }
  }

  return -1;
};

test([{ f: f0, expected: 357 }]);

benchmark(f0, f1);
