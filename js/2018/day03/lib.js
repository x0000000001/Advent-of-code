import { benchmark, test } from "./main.js";

class Square {
  constructor(x_top_left, y_top_left, width, height) {
    this.x0 = x_top_left;
    this.x1 = x_top_left + width - 1;
    this.y0 = y_top_left;
    this.y1 = y_top_left + height - 1;
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
  let id = parseInt(w[0].substring(1));
  let [x, y] = w[2]
    .substring(0, w[2].length - 1)
    .split(",")
    .map((x) => parseInt(x));

  let [width, height] = w[3].split("x").map((x) => parseInt(x));

  return new Square(x, y, width, height);
};

export const format = (lines) => {
  return lines.map(get_request);
};

const get_area = (squares) => {
  if (squares.length === 0) {
    return 0;
  }

  const current = squares[0];
  const others = squares.slice(1);

  const area_others = get_area(others);
  const intersections = others
    .map((sq) => current.intersection(sq))
    .filter((sq) => sq !== null);
  const area_intersections = get_area(intersections);

  return area_others + current.area - area_intersections;
};

const f0 = (input) => {
  let inters = [];
  for (let i = 0; i < input.length; i++) {
    const x = input[i];
    for (let j = i + 1; j < input.length; j++) {
      const y = input[j];
      const inter = x.intersection(y);
      if (inter !== null) {
        inters.push(inter);
      }
    }
  }
  return get_area(inters);
};

const f1 = (input) => {
  for (let i = 0; i < input.length; i++) {
    const x = input[i];
    let has_collide = false;
    for (let j = 0; j < input.length; j++) {
      if (i === j) {
        continue;
      }
      const y = input[j];
      if (x.intersection(y) !== null) {
        has_collide = true;
        break;
      }
    }

    if (!has_collide) {
      return i + 1;
    }
  }
};

test([
  { f: get_area, expected: 32 },
  { f: f0, expected: 4 },
  { f: f1, expected: 3 },
]);

benchmark(f0, f1);
