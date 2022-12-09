"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const format = (lines) => {
    const get_dir = (l) => {
        let words = l.split(" ");
        return [words[0], parseInt(words[1])];
    };
    return { directions: lines.map(get_dir) };
};
exports.format = format;
const count_tail_positions = ({ directions }, n) => {
    let rope = Array.from({ length: n }, () => {
        return { x: 0, y: 0 };
    });
    let visited = new Set();
    const signature = (p) => p.x * 100000 + p.y;
    visited.add(signature(rope[n - 1]));
    directions.forEach(([dir, amplitude]) => {
        for (let i = 0; i < amplitude; i++) {
            switch (dir) {
                case "R":
                    rope[0].y += 1;
                    break;
                case "L":
                    rope[0].y -= 1;
                    break;
                case "U":
                    rope[0].x -= 1;
                    break;
                case "D":
                    rope[0].x += 1;
                    break;
                default:
                    throw new Error("no");
            }
            for (let j = 1; j < n; j++) {
                if (Math.abs(rope[j].x - rope[j - 1].x) > 1 ||
                    Math.abs(rope[j].y - rope[j - 1].y) > 1) {
                    if (rope[j - 1].x > rope[j].x) {
                        rope[j].x += 1;
                    }
                    else if (rope[j - 1].x < rope[j].x) {
                        rope[j].x -= 1;
                    }
                    if (rope[j - 1].y > rope[j].y) {
                        rope[j].y += 1;
                    }
                    else if (rope[j - 1].y < rope[j].y) {
                        rope[j].y -= 1;
                    }
                }
            }
            visited.add(signature(rope[n - 1]));
            // console.log(head, tail);
        }
    });
    return visited.size;
};
const f0 = ({ directions }) => {
    return count_tail_positions({ directions }, 2);
};
const f1 = ({ directions }) => {
    return count_tail_positions({ directions }, 10);
};
(0, main_js_1.test)([
    { f: f0, expected: 13 },
    { f: f1, expected: 36, file: "test_input1.txt" },
]);
// 6066 too low
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map