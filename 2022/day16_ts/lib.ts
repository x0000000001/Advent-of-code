import { cp } from "fs";
import { benchmark, test } from "./main.js";

// valves, distances between valves, id of AA
type InputType = [Map<Valve, ValveData>, number];

type ValveData = {
  flowRate: number;
  links: number[];
};

type OutputType = number;
type ValveSet = number;

// [Valve id, distance][]
const direct_non_nul_neighbors = (
  valves: Map<Valve, ValveData>,
  start: Valve
): [Valve, number][] => {
  let direct_neighbors = [];
  let scores = new Map();
  let queue = [];
  queue.push([start, 0]);
  scores.set(start, 0);

  while (queue.length > 0) {
    queue.sort(([, score0], [, score1]) => score0 - score1);
    let [valve, score] = queue.pop();

    if (valve !== start && valves.get(valve).flowRate > 0) {
      direct_neighbors.push([valve, score]);
      continue;
    }

    for (const n of valves.get(valve).links) {
      if (!scores.has(n) || scores.get(n) < score + 1) {
        continue;
      }

      scores.set(n, score + 1);
      queue.push([n, score + 1]);
    }
  }

  return direct_neighbors;
};

export const format = (lines: string[]): InputType => {
  let valve_name_to_number = new Map();
  let current_valve_num = 0;

  const valve_number = (s: string): number => {
    if (valve_name_to_number.has(s)) {
      return valve_name_to_number.get(s);
    } else {
      valve_name_to_number.set(s, current_valve_num);
      current_valve_num++;
      return current_valve_num - 1;
    }
  };

  const get_valve = (l: string): [number, ValveData] => {
    let w = l.split(" ");
    let flowRate = parseInt(w[4].substring(5, w[4].length - 1));
    let links = w.slice(9).map((word) => valve_number(word.replace(",", "")));

    return [
      valve_number(w[1]),
      {
        flowRate,
        links,
      },
    ];
  };

  let m = new Map();
  lines.map(get_valve).forEach(([s, v]) => m.set(s, v));

  return [m, valve_name_to_number.get("AA")];
};

const valve_set_add = (s: ValveSet, v: number): number => {
  if (v == null) {
    return s;
  }

  return s | (1 << v);
};

const valve_set_remove = (s: ValveSet, v: number): number => {
  return s & ~(1 << v);
};

const valve_set_contains = (s: ValveSet, v: number): boolean => {
  return (s >> v) % 2 == 1;
};

const hash_state = (valve: Valve, time: Time): State => {
  return valve * 10000 + time;
};

type State = number;
type Score = number;
type Valve = number;
type Time = number;

const f0 = ([valves, valve_aa_index]: InputType): OutputType => {
  let all_non_zero_valves_opened = 0;

  for (const k of valves.keys()) {
    if (valves.get(k).flowRate !== 0) {
      all_non_zero_valves_opened = valve_set_add(all_non_zero_valves_opened, k);
    }
  }

  let scores: Map<State, Map<ValveSet, Score>> = new Map();
  let queue: [Valve, Time, ValveSet, Score][] = [];

  let hash = hash_state(valve_aa_index, 30);

  scores.set(hash, new Map());
  scores.get(hash).set(0, 0);
  queue.push([valve_aa_index, 30, 0, 0]);
  let max_score = 0;

  while (queue.length !== 0) {
    // queue.sort(
    //   ([, time0, , score0], [, time1, , score1]) =>
    //     (time1 - time0) * 10000 + (score0 - score1)
    // );
    let [valve, time, set, score] = queue.pop();

    if (time === 0) {
      max_score = Math.max(max_score, score);
      continue;
    }
    if (set == all_non_zero_valves_opened) {
      // all valves opened
      max_score = Math.max(max_score, score);
      continue;
    }

    let neighbors = [];

    if (valves.get(valve).flowRate !== 0 && !valve_set_contains(set, valve)) {
      // opening valve
      neighbors.push([
        valve,
        time - 1,
        valve_set_add(set, valve),
        score + (time - 1) * valves.get(valve).flowRate,
      ]);
    }

    // visiting neighbors
    for (const n of valves.get(valve).links) {
      neighbors.push([n, time - 1, set, score]);
    }

    for (const neigh of neighbors) {
      let hash = hash_state(neigh[0], neigh[1]);
      let existing_scores;
      if (!scores.has(hash)) {
        scores.set(hash, new Map());
      }
      existing_scores = scores.get(hash);

      if (
        existing_scores.has(neigh[2]) &&
        existing_scores.get(neigh[2]) >= neigh[3]
      ) {
        continue;
      }

      scores.get(hash).set(neigh[2], neigh[3]);
      queue.push(neigh);
    }
  }

  return max_score;
};

const hash_state1 = (valve0: Valve, valve1: Valve, time: Time): State => {
  // both players are equivalent : (0,1,x) <=> (1,0,x)
  return (
    Math.max(valve0, valve1) * 100000 + Math.min(valve0, valve1) * 100 + time
  );
};

const f1 = ([valves, valve_aa_index]: InputType): OutputType => {
  let all_non_zero_valves_opened = 0;

  for (const k of valves.keys()) {
    if (valves.get(k).flowRate !== 0) {
      all_non_zero_valves_opened = valve_set_add(all_non_zero_valves_opened, k);
    }
  }

  let scores: Map<State, Map<ValveSet, Score>> = new Map();
  let queue: [Valve, Valve, Time, ValveSet, Score][] = [];

  let hash = hash_state1(valve_aa_index, valve_aa_index, 26);

  scores.set(hash, new Map());
  scores.get(hash).set(0, 0);
  queue.push([valve_aa_index, valve_aa_index, 26, 0, 0]);
  let max_score = 0;

  while (queue.length !== 0) {
    // queue.sort(
    //   ([, time0, , score0], [, time1, , score1]) =>
    //     (time1 - time0) * 10000 + (score0 - score1)
    // );
    let [valve0, valve1, time, set, score] = queue.pop();

    if (time === 0) {
      max_score = Math.max(max_score, score);
      continue;
    }

    if (set == all_non_zero_valves_opened) {
      // all valves opened
      max_score = Math.max(max_score, score);
      continue;
    }

    const get_possibilities = (v: Valve): [Valve, Score, Valve][] => {
      // first valve is the next position
      // score is the produced score
      // second valve is the potential opened one
      let possibilities: [Valve, Score, Valve][] = [];

      if (valves.get(v).flowRate !== 0 && !valve_set_contains(set, v)) {
        // opening valve
        possibilities.push([v, (time - 1) * valves.get(v).flowRate, v]);
      }

      // visiting neighbors
      for (const n of valves.get(v).links) {
        possibilities.push([n, 0, null]);
      }

      return possibilities;
    };

    let neighbors: [Valve, Valve, Time, ValveSet, Score][] = [];

    for (const [v0, score0, opened0] of get_possibilities(valve0)) {
      for (const [v1, score1, opened1] of get_possibilities(valve1)) {
        if (opened0 !== null && opened0 === opened1) {
          continue;
        }

        neighbors.push([
          v0,
          v1,
          time - 1,
          valve_set_add(valve_set_add(set, opened0), opened1),
          score + score0 + score1,
        ]);
      }
    }

    for (const neigh of neighbors) {
      let hash = hash_state1(neigh[0], neigh[1], neigh[2]);
      let existing_scores;
      if (!scores.has(hash)) {
        scores.set(hash, new Map());
      }
      existing_scores = scores.get(hash);

      if (
        existing_scores.has(neigh[3]) &&
        existing_scores.get(neigh[3]) >= neigh[4]
      ) {
        continue;
      }

      scores.get(hash).set(neigh[3], neigh[4]);
      queue.push(neigh);
    }
  }

  return max_score;
};

test([
  { f: f0, expected: 1651 },
  { f: f1, expected: 1707 },
]);

// 2166 too low
benchmark(f0, f1);
