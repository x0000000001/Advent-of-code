"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
// [Valve id, distance][]
const non_nul_neighbors = (valves, start) => {
    let non_nul_neighs = [];
    let scores = new Map();
    let queue = [];
    queue.push([start, 0]);
    scores.set(start, 0);
    while (queue.length > 0) {
        queue.sort(([, score0], [, score1]) => score1 - score0);
        let [valve, score] = queue.pop();
        if (valve !== start && valves.get(valve).flowRate > 0) {
            non_nul_neighs.push([valve, score]);
            continue; // line to comment to remove direct criteria
        }
        for (const [n, _] of valves.get(valve).links) {
            if (scores.has(n) && scores.get(n) <= score + 1) {
                continue;
            }
            scores.set(n, score + 1);
            queue.push([n, score + 1]);
        }
    }
    return non_nul_neighs;
};
const format = (lines) => {
    let valve_name_to_id = new Map();
    let current_valve_num = 0;
    const valve_number = (s) => {
        if (valve_name_to_id.has(s)) {
            return valve_name_to_id.get(s);
        }
        else {
            valve_name_to_id.set(s, current_valve_num);
            current_valve_num++;
            return current_valve_num - 1;
        }
    };
    const get_valve = (l) => {
        let w = l.split(" ");
        let flowRate = parseInt(w[4].substring(5, w[4].length - 1));
        let links = w
            .slice(9)
            .map((word) => [valve_number(word.replace(",", "")), 1]);
        return [
            valve_number(w[1]),
            {
                flowRate,
                links,
            },
        ];
    };
    let m = new Map();
    lines.map(get_valve).forEach(([v, vd]) => m.set(v, vd));
    // temporarily, so that "AA" it is not suppressed when reducing
    m.get(valve_name_to_id.get("AA")).flowRate = 1;
    // reducing map
    let new_to_old_ids = new Map();
    let old_to_new_ids = new Map();
    let current_id = 0;
    for (const [valve, valve_data] of m.entries()) {
        if (valve_data.flowRate > 0) {
            old_to_new_ids.set(valve, current_id);
            new_to_old_ids.set(current_id, valve);
            current_id++;
        }
    }
    let final_valves = new Map();
    for (const entry of new_to_old_ids.entries()) {
        let neighbors = non_nul_neighbors(m, entry[1]).map(([old_id, distance]) => [old_to_new_ids.get(old_id), distance]);
        final_valves.set(entry[0], {
            flowRate: m.get(entry[1]).flowRate,
            links: neighbors,
        });
    }
    let new_aa_id = old_to_new_ids.get(valve_name_to_id.get("AA"));
    // restoring flowrate of 0 at AA
    final_valves.get(new_aa_id).flowRate = 0;
    return [final_valves, new_aa_id];
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
        let [valve, time, set, score] = queue.pop();
        if (time < 0) {
            continue;
        }
        max_score = Math.max(max_score, score);
        if (set === all_non_zero_valves_opened) {
            continue;
        }
        let neighbors = [];
        // visiting neighbors
        for (const [n, distance] of valves.get(valve).links) {
            if (time <= distance) {
                continue;
            }
            // neighbors.push([n, time - distance, set, score]);
            if (!valve_set_contains(set, n)) {
                neighbors.push([
                    n,
                    time - distance - 1,
                    valve_set_add(set, n),
                    score + valves.get(n).flowRate * (time - distance - 1),
                ]);
            }
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
const hash_state1 = (valve, time, first_player) => {
    return (valve * 1000 + time) * (first_player ? 1 : -1);
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
    let hash = hash_state1(valve_aa_index, 26, true);
    scores.set(hash, new Map());
    scores.get(hash).set(0, 0);
    queue.push([valve_aa_index, 26, true, 0, 0]);
    let max_score = 0;
    while (queue.length !== 0) {
        let [valve, time, first_player, set, score] = queue.pop();
        if (time < 0) {
            continue;
        }
        max_score = Math.max(max_score, score);
        if (set === all_non_zero_valves_opened) {
            continue;
        }
        let neighbors = [];
        // visiting neighbors
        for (const [n, distance] of valves.get(valve).links) {
            if (time <= distance) {
                continue;
            }
            // neighbors.push([
            //   n,
            //   time - distance,
            //   first_player,
            //   valve_set_add(set, n),
            //   score,
            // ]);
            if (!valve_set_contains(set, n)) {
                neighbors.push([
                    n,
                    time - distance - 1,
                    first_player,
                    valve_set_add(set, n),
                    score + valves.get(n).flowRate * (time - distance - 1),
                ]);
            }
        }
        if (neighbors.length === 0 && first_player) {
            queue.push([valve_aa_index, 26, false, set, score]);
            continue;
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
// 2233 too low
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map