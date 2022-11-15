import { benchmark, test } from "./main.js";

export const format = (lines) => {
  const process_line = (line) => {
    let words = line.split(" ");
    let [year, month, day] = words[0]
      .substring(1)
      .split("-")
      .map((x) => parseInt(x));
    let [hour, minute] = words[1]
      .substring(0, words[1].length - 1)
      .split(":")
      .map((x) => parseInt(x));
    let data = {
      time:
        minute +
        hour * 60 +
        day * 60 * 24 +
        month * 60 * 24 * 31 +
        year * 60 * 24 * 31 * 365,
    };

    data.minute = minute;

    switch (words[2]) {
      case "Guard":
        data.type = "shift";
        data.guard = parseInt(words[3].substring(1));
        break;
      case "wakes":
        data.type = "wakesup";
        break;
      case "falls":
        data.type = "fallsasleep";
        break;
      default:
        throw `unvalid data : ${words[2]}`;
    }

    return data;
  };
  return lines.map(process_line).sort((a, b) => a.time - b.time);
};

const most_slept_minute = (times) => {
  let minutes_slept = new Array(60).fill(0);

  times.forEach(([start, end]) => {
    for (let i = start; i < end; i++) {
      minutes_slept[i]++;
    }
  });

  return minutes_slept
    .map((x, i) => [x, i])
    .sort(([xa], [xb]) => xa - xb)
    .pop();
};

const guards_times = (input) => {
  let guards_times = {};
  let current_guard = null;
  let sleep_start_minute = null;

  for (let i = 0; i < input.length; i++) {
    switch (input[i].type) {
      case "shift":
        current_guard = input[i].guard;
        break;
      case "wakesup":
        if (!(current_guard in guards_times)) {
          guards_times[current_guard] = [];
        }
        guards_times[current_guard].push([sleep_start_minute, input[i].minute]);
        break;
      case "fallsasleep":
        sleep_start_minute = input[i].minute;
        break;
    }
  }

  return guards_times;
};

const f0 = (input) => {
  const gt = guards_times(input);
  const max_amplitude_id = parseInt(
    Object.entries(gt)
      .map(([name, times]) => [
        name,
        times.map(([start, end]) => end - start).reduce((a, b) => a + b, 0),
      ])
      .sort(([, a], [, b]) => {
        return a - b;
      }, (-1, -1))
      .pop()[0]
  );

  return max_amplitude_id * most_slept_minute(gt[max_amplitude_id])[1];
};

const f1 = (input) => {
  const gt = guards_times(input);
  const msm = Object.entries(gt)
    .map(([name, times]) => [name, most_slept_minute(times)])
    .sort(([, [xa, min_index_a]], [, [xb, min_index_b]]) => xa - xb)
    .pop();

  return msm[0] * msm[1][1];
};

test([
  { f: f0, expected: 240 },
  { f: f1, expected: 4455 },
]);

benchmark(f0, f1);
