import { benchmark, test } from "./main.js";

type InputType = [string, string][];

export const format = (lines: string[]) => {
  return lines.map((l) => {
    let words: string[] = l.split(" ");
    return [words[1], words[7]];
  });
};

const get_relations = (input): [string, string[]][] => {
  let obligations = {};

  input.forEach(([x, y]) => {
    if (!(x in obligations)) {
      obligations[x] = [];
    }

    if (!(y in obligations)) {
      obligations[y] = [];
    }

    obligations[y].push(x);
  });

  return Object.entries(obligations);
};

const f0 = (input: InputType) => {
  let relations = get_relations(input);
  let res = "";

  while (relations.length > 0) {
    let candidates = [];
    let candidates_ids = {};
    for (let index = 0; index < relations.length; index++) {
      const [key, obl]: [string, string[]] = relations[index];
      if (obl.length === 0) {
        candidates.push(key);
        candidates_ids[key] = index;
      }
    }

    candidates.sort();
    let key = candidates.shift();
    relations.splice(candidates_ids[key], 1);
    res += key;

    for (let i = 0; i < relations.length; i++) {
      let remove_index = relations[i][1].indexOf(key);
      if (remove_index != -1) {
        relations[i][1].splice(remove_index, 1);
      }
    }
  }

  return res;
};

const time_for_step = (c, offset) => {
  return c.charCodeAt(0) - 64 + offset;
};

const compute_time = (
  relations: [string, string[]][],
  time_offset: number,
  workers_count: number
): number => {
  let workers_tasks: [string, number][] = new Array(workers_count).fill(null);
  let time = 0;

  while (
    relations.length > 0 ||
    workers_tasks.map((x) => x !== null).reduce((a, b) => a || b, false)
  ) {
    let candidates = [];
    for (let index = 0; index < relations.length; index++) {
      const [key, obl]: [string, string[]] = relations[index];
      if (obl.length === 0) {
        candidates.push(key);
      }
    }

    candidates.sort();

    for (let index = 0; index < workers_count; index++) {
      if (candidates.length === 0) {
        break;
      }

      if (workers_tasks[index] === null) {
        let key = candidates.shift();
        let id = -1;
        for (let i = 0; i < relations.length; i++) {
          const element = relations[i];
          if (element[0] == key) {
            id = i;
            break;
          }
        }

        relations.splice(id, 1);
        workers_tasks[index] = [key, time_for_step(key, time_offset)];
      }
    }

    for (let index = 0; index < workers_count; index++) {
      if (workers_tasks[index] === null) {
        continue;
      }

      workers_tasks[index][1]--;
      if (workers_tasks[index][1] === 0) {
        const key = workers_tasks[index][0];
        for (let i = 0; i < relations.length; i++) {
          let remove_index = relations[i][1].indexOf(key);
          if (remove_index != -1) {
            relations[i][1].splice(remove_index, 1);
          }
        }
        workers_tasks[index] = null;
      }
    }

    time++;
  }

  return time;
};

const f1 = (input) => {
  return compute_time(get_relations(input), 60, 5);
};

test([
  { f: f0, expected: "CABDFE" },
  { f: (x) => compute_time(get_relations(x), 0, 2), expected: 15 },
]);

benchmark(f0, f1);
