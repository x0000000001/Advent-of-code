"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const format = (lines) => {
    let valve_name_to_number = new Map();
    let current_valve_num = 0;
    const valve_number = (s) => {
        if (valve_name_to_number.has(s)) {
            return valve_name_to_number.get(s);
        }
        else {
            valve_name_to_number.set(s, current_valve_num);
            current_valve_num++;
            return current_valve_num - 1;
        }
    };
    const get_valve = (l) => {
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
exports.format = format;
const valve_set_add = (s, v) => {
    if (v == null) {
        return s;
    }
    return s | (1 << v);
};
const valve_set_remove = (s, v) => {
    return s & ~(1 << v);
};
const valve_set_contains = (s, v) => {
    return (s >> v) % 2 == 1;
};
const hash_state = (valve, time) => {
    return valve * 10000 + time;
};
const f0 = ([valves, valve_aa_index]) => {
    let all_non_zero_valves_opened = 0;
    for (const k of valves.keys()) {
        if (valves.get(k).flowRate !== 0) {
            all_non_zero_valves_opened = valve_set_add(all_non_zero_valves_opened, k);
        }
    }
    let scores = new Map();
    let queue = [];
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
            if (existing_scores.has(neigh[2]) &&
                existing_scores.get(neigh[2]) >= neigh[3]) {
                continue;
            }
            scores.get(hash).set(neigh[2], neigh[3]);
            queue.push(neigh);
        }
    }
    return max_score;
};
const hash_state1 = (valve0, valve1, time) => {
    // both players are equivalent : (0,1,x) <=> (1,0,x)
    return (Math.max(valve0, valve1) * 100000 + Math.min(valve0, valve1) * 100 + time);
};
const f1 = ([valves, valve_aa_index]) => {
    let all_non_zero_valves_opened = 0;
    for (const k of valves.keys()) {
        if (valves.get(k).flowRate !== 0) {
            all_non_zero_valves_opened = valve_set_add(all_non_zero_valves_opened, k);
        }
    }
    let scores = new Map();
    let queue = [];
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
        const get_possibilities = (v) => {
            // first valve is the next position
            // score is the produced score
            // second valve is the potential opened one
            let possibilities = [];
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
        let neighbors = [];
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
            if (existing_scores.has(neigh[3]) &&
                existing_scores.get(neigh[3]) >= neigh[4]) {
                continue;
            }
            scores.get(hash).set(neigh[3], neigh[4]);
            queue.push(neigh);
        }
    }
    return max_score;
};
(0, main_js_1.test)([
    { f: f0, expected: 1651 },
    { f: f1, expected: 1707 },
]);
// 2166 too low
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map