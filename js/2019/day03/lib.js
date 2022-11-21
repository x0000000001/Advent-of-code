import { benchmark, test } from "./main.js";

export const format = (lines) => {
  const get_nums = (l) =>
    l.split(",").map((x) => [x.charAt(0), parseInt(x.substring(1))]);
  return [get_nums(lines[0]), get_nums(lines[1])];
};

const locations = (instrs, pos = [0, 0], visited = new Set(), i = 0) => {
  visited.add(structuredClone(pos));

  if (i === instrs.length) {
    return visited;
  }

  let [cmd, n] = instrs[i];
  switch (cmd) {
    case "R":
      pos[0] += n;
      break;
    case "L":
      pos[0] -= n;
      break;
    case "U":
      pos[1] += n;
      break;
    case "D":
      pos[1] -= n;
      break;
    default:
      throw "invalid instruction";
  }

  return locations(instrs, pos, visited, i + 1);
};

const manhattan_distance = (pos) => {
  return Math.abs(pos[0]) + Math.abs(pos[1]);
};

const collision = ([x00, y00], [x01, y01], [x10, y10], [x11, y11]) => {
  if (x00 == x01) {
    let [startx, endx] = [Math.min(x10, x11), Math.max(x10, x11)];
    let [starty, endy] = [Math.min(y00, y01), Math.max(y00, y01)];
    if (x00 >= startx && x00 <= endx && y10 >= starty && y10 <= endy) {
      return [x00, y10];
    } else {
      return null;
    }
  } else {
    let [startx, endx] = [Math.min(x00, x01), Math.max(x00, x01)];
    let [starty, endy] = [Math.min(y10, y11), Math.max(y10, y11)];
    if (x10 >= startx && x10 <= endx && y00 >= starty && y00 <= endy) {
      return [x10, y00];
    } else {
      return null;
    }
  }
};

const collisions = (locations0, locations1) => {
  let cs = [];
  for (let i = 0; i < locations0.length - 1; i++) {
    for (let j = 0; j < locations1.length - 1; j++) {
      let c = collision(
        locations0[i],
        locations0[i + 1],
        locations1[j],
        locations1[j + 1]
      );

      if (c != null) {
        cs.push(c);
      }
    }
  }

  return cs;
};

const f0 = ([w0, w1]) => {
  let [loc0, loc1] = [[...locations(w0)], [...locations(w1)]];
  return Math.min(
    ...collisions(loc0, loc1)
      .map(manhattan_distance)
      .filter((x) => x != 0)
  );
};

const steps_to_get_to = (
  [x, y],
  instrs,
  pos = [0, 0],
  instr_index = 0,
  steps = 0
) => {
  if (instr_index === instrs.length) {
    return NaN;
  }

  let [cmd, n] = instrs[instr_index];
  switch (cmd) {
    case "R":
      if (y == pos[1] && x >= pos[0] && x <= pos[0] + n) {
        return steps + x - pos[0];
      }
      pos[0] += n;
      break;
    case "L":
      if (y == pos[1] && x <= pos[0] && x >= pos[0] - n) {
        return steps + pos[0] - x;
      }
      pos[0] -= n;
      break;
    case "U":
      if (x == pos[0] && y >= pos[1] && y <= pos[1] + n) {
        return steps + y - pos[1];
      }
      pos[1] += n;
      break;
    case "D":
      if (x == pos[0] && y <= pos[1] && y >= pos[1] - n) {
        return steps + pos[1] - y;
      }
      pos[1] -= n;
      break;
    default:
      throw `invalid instruction ${cmd}`;
  }

  return steps_to_get_to([x, y], instrs, pos, instr_index + 1, steps + n);
};

const cost_of_inter = (inter, w0, w1) => {
  return steps_to_get_to(inter, w0) + steps_to_get_to(inter, w1);
};

const f1 = ([w0, w1]) => {
  let [loc0, loc1] = [[...locations(w0)], [...locations(w1)]];
  return Math.min(
    ...collisions(loc0, loc1)
      .map((inter) => cost_of_inter(inter, w0, w1))
      .filter((x) => x != 0)
  );
};

test([
  { f: f0, expected: 6 },
  { f: f0, expected: 159, file: "test_input1.txt" },
  { f: f1, expected: 610, file: "test_input1.txt" },
]);

benchmark(f0, f1);
