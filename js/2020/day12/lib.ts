import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  return lines.map((l) => [l.charAt(0), parseInt(l.slice(1))]);
};

const f0 = (input) => {
  let [x, y] = [0, 0];
  let orientation = 1;

  for (let i = 0; i < input.length; i++) {
    let [cmd, val] = input[i];
    switch (cmd) {
      case "F":
        switch (orientation) {
          case 0:
            y += val;
            break;
          case 1:
            x += val;
            break;
          case 2:
            y -= val;
            break;
          case 3:
            x -= val;
            break;
        }
        break;

      case "N":
        y += val;
        break;

      case "S":
        y -= val;
        break;

      case "E":
        x += val;
        break;

      case "W":
        x -= val;
        break;

      case "R":
        orientation = (orientation + val / 90) % 4;
        break;

      case "L":
        orientation = (orientation - val / 90 + 4) % 4;
        break;
    }
  }

  return Math.abs(x) + Math.abs(y);
};

const f1 = (input) => {
  let [x, y] = [0, 0];
  let [waypointx, waypointy] = [10, 1];

  for (let i = 0; i < input.length; i++) {
    let [cmd, val] = input[i];
    switch (cmd) {
      case "F":
        x += val * waypointx;
        y += val * waypointy;
        break;

      case "N":
        waypointy += val;
        break;

      case "S":
        waypointy -= val;
        break;

      case "E":
        waypointx += val;
        break;

      case "W":
        waypointx -= val;
        break;

      case "R":
        switch (val / 90) {
          case 0:
            break;
          case 1:
            let temp1 = waypointx;
            waypointx = waypointy;
            waypointy = -temp1;
            break;
          case 2:
            waypointx = -waypointx;
            waypointy = -waypointy;
            break;
          case 3:
            let temp3 = waypointx;
            waypointx = -waypointy;
            waypointy = temp3;
            break;
        }
        break;

      case "L":
        switch (val / 90) {
          case 0:
            break;
          case 1:
            let temp1 = waypointx;
            waypointx = -waypointy;
            waypointy = temp1;
            break;
          case 2:
            waypointx = -waypointx;
            waypointy = -waypointy;
            break;
          case 3:
            let temp3 = waypointx;
            waypointx = waypointy;
            waypointy = -temp3;
            break;
        }
        break;
    }
  }

  return Math.abs(x) + Math.abs(y);
};

test([
  { f: f0, expected: 25 },
  { f: f1, expected: 286 },
]);

benchmark(f0, f1);
