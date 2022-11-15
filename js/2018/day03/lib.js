import { benchmark, test } from "./main.js";

class Square {
  constructor(x_top_left, y_top_left, width, height) {
    this.x0 = x_top_left;
    this.x1 = x_top_left + width;
    this.y0 = y_top_left;
    this.y1 = y_top_left + height;
    this.width = width;
    this.height = height;
  }

  get area() {
    return this.width * this.height;
  }

  intersection(other) {
    if (
      other.x0 > this.x1 ||
      other.x1 < this.x0 ||
      other.y0 > this.y1 ||
      other.y1 < this.y0
    ) {
      return null;
    }

    let [xmin, xmax] = [
      Math.max(other.x0, this.x0),
      Math.min(other.x1, this.x1),
    ];
    let [ymin, ymax] = [
      Math.max(other.y0, this.y0),
      Math.min(other.y1, this.y1),
    ];

    return new Square(xmin, ymin, xmax - xmin + 1, ymax - ymin + 1);
  }
}

const get_request = (line) => {
  let w = line.split(" ");
  let id = parseInt(w(0).substring(1));
  let [x, y] = w[2]
    .substring(0, w[2].length - 1)
    .split(",")
    .map((x) => parseInt(x));

  let [width, height] = w[3].split("x").map((x) => parseInt(x));

  return [id, new Square(x, y, width, height)];
};

export const format = (lines) => {
  return lines.map(get_request);
};

const get_intersections = (squares) => {
  if (squares.isEmpty()) {
    return { area: 0, intersections: [] };
  }

  let current = squares[0];
  let remaining = squares.slice(1);

  let inters_with_current = get_intersections(
    remaining.map(current.intersection)
  );
  let inters_of_remaining = get_intersections(remaining);
  let recurrent_inters = get_intersections(
    inters_with_current[intersections].concat(
      inters_of_remaining[intersections]
    )
  );

  return;
};

const f0 = (input) => {
  return 0;
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 4 },
  { f: f1, expected: 0 },
]);

// benchmark(f0, f1);
