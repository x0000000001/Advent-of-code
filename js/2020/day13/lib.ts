import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return [
    parseInt(lines[0]),
    lines[1].split(",").map((c) => (c == "x" ? null : parseInt(c))),
  ];
};

const f0 = ([arrival_time, buses]) => {
  return buses
    .map((id) =>
      id == null
        ? [Number.MAX_SAFE_INTEGER, -1]
        : [(Math.floor(arrival_time / id) + 1) * id - arrival_time, id]
    )
    .reduce(
      ([waitingtime, id], [min, idmin]) =>
        waitingtime < min ? [waitingtime, id] : [min, idmin],
      [Number.MAX_SAFE_INTEGER, -1]
    )
    .reduce((a, b) => a * b, 1);
};

const is_valid = (t, buses): boolean => {
  for (let i = 0; i < buses.length; i++) {
    if (buses[i] === null) continue;
    if ((t + i) % buses[i] !== 0) {
      return false;
    }
  }
  return true;
};

const f1 = ([_, buses]) => {
  let t = 0;
  let acc = 1;
  let conditions = buses.map((x, i) => [x, i]).filter(([x, _]) => x !== null);
  conditions.sort(([x0, i0], [x1, i1]) => x0 - x1);

  for (let index = 0; index < conditions.length; index++) {
    const [id, i] = conditions[index];
    if (id === null) continue;

    while ((t + i) % id !== 0) {
      t += acc;
    }
    acc *= id;
  }

  return t;
};

test([
  { f: f0, expected: 295 },
  { f: f1, expected: 1068781 },
  { f: f1, expected: 3417, file: "test_input1.txt" },
]);

benchmark(f0, f1);
