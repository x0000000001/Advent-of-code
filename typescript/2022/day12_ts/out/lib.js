"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const format = (lines) => {
    let startx, starty, endx, endy;
    let [h, w] = [lines.length, lines[0].length];
    let map = Array.from({ length: h }, () => Array.from({ length: w }, () => 0));
    for (let i = 0; i < h; i++) {
        let chars = lines[i].split("");
        for (let j = 0; j < w; j++) {
            switch (chars[j]) {
                case "S":
                    startx = i;
                    starty = j;
                    map[i][j] = 0;
                    break;
                case "E":
                    endx = i;
                    endy = j;
                    map[i][j] = 25;
                    break;
                default:
                    map[i][j] = chars[j].charCodeAt(0) - 97;
                    break;
            }
        }
    }
    return { map, startx, endx, starty, endy };
};
exports.format = format;
const Djisktra = ({ map, startx, starty }, end_condition, are_neighbors_condition) => {
    let queue = [[0, startx, starty]];
    let [h, w] = [map.length, map[0].length];
    let scores = Array.from({ length: h }, () => Array.from({ length: w }, () => Number.MAX_SAFE_INTEGER));
    scores[startx][starty] = 0;
    while (queue.length != 0) {
        queue.sort((a, b) => b[0] - a[0]);
        let [score, x, y] = queue.pop();
        if (end_condition(x, y)) {
            return score;
        }
        let candidates = [];
        let new_score = score + 1;
        let height = map[x][y];
        if (x > 0 &&
            scores[x - 1][y] > new_score &&
            are_neighbors_condition(x, y, x - 1, y)) {
            candidates.push([x - 1, y]);
        }
        if (y > 0 &&
            scores[x][y - 1] > new_score &&
            are_neighbors_condition(x, y, x, y - 1)) {
            candidates.push([x, y - 1]);
        }
        if (x < h - 1 &&
            scores[x + 1][y] > new_score &&
            are_neighbors_condition(x, y, x + 1, y)) {
            candidates.push([x + 1, y]);
        }
        if (y < w - 1 &&
            scores[x][y + 1] > new_score &&
            are_neighbors_condition(x, y, x, y + 1)) {
            candidates.push([x, y + 1]);
        }
        for (const [candidatex, candidatey] of candidates) {
            scores[candidatex][candidatey] = new_score;
            queue.push([new_score, candidatex, candidatey]);
        }
    }
    return -1;
};
// Djisktra
const f0 = (input) => {
    let end_condition = (x, y) => x == input.endx && y == input.endy;
    7;
    let are_neighbors_condition = (xorigin, yorigin, xcandidate, ycandidate) => input.map[xcandidate][ycandidate] <= input.map[xorigin][yorigin] + 1;
    return Djisktra(input, end_condition, are_neighbors_condition);
};
// Djisktra - unknown target
const f1 = (input) => {
    let end_condition = (x, y) => input.map[x][y] == 0;
    let are_neighbors_condition = (xorigin, yorigin, xcandidate, ycandidate) => input.map[xcandidate][ycandidate] >= input.map[xorigin][yorigin] - 1;
    input.startx = input.endx;
    input.starty = input.endy;
    return Djisktra(input, end_condition, are_neighbors_condition);
};
(0, main_js_1.test)([
    { f: f0, expected: 31 },
    { f: f1, expected: 29 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map