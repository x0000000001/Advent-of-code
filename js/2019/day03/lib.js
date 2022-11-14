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
      pos[0]++;
      break;
    case "L":
      pos[0]--;
      break;
    case "U":
      pos[1]++;
      break;
    case "D":
      pos[1]--;
      break;
    default:
      throw "invalid instruction";
  }

  return locations(instrs, pos, visited, i + 1);
};

const manhattan_distance = (pos) => {
  return Math.abs(pos[0]) + Math.abs(pos[1]);
};

const f0 = ([w0, w1]) => {
  let [loc0, loc1] = [[...locations(w0)], [...locations(w1)]];
  let y = loc0.filter((x) => x in loc1);
  return Math.min(...loc0.filter((x) => x in loc1).map(manhattan_distance));
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 6 },
  { f: f1, expected: 0 },
]);

// benchmark(f0, f1);
