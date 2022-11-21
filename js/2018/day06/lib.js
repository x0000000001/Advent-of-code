import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((l) => l.split(",").map((x) => parseInt(x)));
};

const manhattan_distance = ([x0, y0], [x1, y1]) => {
  return Math.abs(x1 - x0) + Math.abs(y1 - y0);
};

const closest_point = (coords, points) => {
  const closest_points = points
    .map((p) => manhattan_distance(coords, p))
    .reduce(
      ([min_val, min_indexs], current_val, current_index) => {
        if (current_val < min_val) {
          return [current_val, [current_index]];
        } else if (current_val == min_val) {
          min_indexs.push(current_index);
          return [min_val, min_indexs];
        } else {
          return [min_val, min_indexs];
        }
      },
      [Number.MAX_SAFE_INTEGER, -1]
    );

  if (closest_points[1].length == 1) {
    return closest_points[1][0];
  } else {
    return null;
  }
};

const f0 = (points) => {
  let [width, height] = points.reduce(
    ([temp_x, temp_y], [x, y]) => [Math.max(x, temp_x), Math.max(y, temp_y)],
    [0, 0]
  );

  let close_points_count = new Array(points.length).fill(0);
  let is_point_valid = new Array(points.length).fill(true);

  for (let x = 0; x < width + 1; x++) {
    for (let y = 0; y < height + 1; y++) {
      const closest_point_index = closest_point([x, y], points);
      if (closest_point_index != null) {
        close_points_count[closest_point_index]++;

        // banning points closest to borders
        if (x == 0 || x == width || y == 0 || y == height) {
          is_point_valid[closest_point_index] = false;
        }
      }
    }
  }

  return close_points_count.reduce((max, b, i) => {
    if (!is_point_valid[i]) {
      return max;
    } else {
      return Math.max(max, b);
    }
  });
};

const safe_region_size = (max_distance, points) => {
  let [width, height] = points.reduce(
    ([temp_x, temp_y], [x, y]) => [Math.max(x, temp_x), Math.max(y, temp_y)],
    [0, 0]
  );

  let count = 0;

  for (let x = 0; x < width + 1; x++) {
    for (let y = 0; y < height + 1; y++) {
      let distances_sum = points
        .map((p) => manhattan_distance([x, y], p))
        .reduce((a, b) => a + b, 0);

      if (distances_sum < max_distance) {
        count++;
      }
    }
  }

  return count;
};

const f1 = (points) => {
  return safe_region_size(10000, points);
};

test([
  { f: f0, expected: 17 },
  { f: (p) => safe_region_size(32, p), expected: 16 },
]);

benchmark(f0, f1);
