"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
const main_js_1 = require("./main.js");
const hash = (x, y) => {
    return 10000 * x + y;
};
const format = (lines) => {
    let walls = new Set();
    const coords_from_lines = (l) => {
        return l.split(" -> ").map((c) => c.split(",").map((x) => parseInt(x)));
    };
    for (const line of lines) {
        let coords = coords_from_lines(line);
        for (let i = 0; i < coords.length - 1; i++) {
            let [minx, maxx] = [
                Math.min(coords[i][0], coords[i + 1][0]),
                Math.max(coords[i][0], coords[i + 1][0]) + 1,
            ];
            let [miny, maxy] = [
                Math.min(coords[i][1], coords[i + 1][1]),
                Math.max(coords[i][1], coords[i + 1][1]) + 1,
            ];
            for (let x = minx; x < maxx; x++) {
                for (let y = miny; y < maxy; y++) {
                    walls.add(hash(x, y));
                }
            }
        }
    }
    return { walls };
};
exports.format = format;
const f0 = ({ walls }) => {
    let t = 0;
    let sand = new Set();
    let maxy = 0;
    walls.forEach((h) => {
        let y = h % 10000;
        if (y > maxy) {
            maxy = y;
        }
    });
    while (true) {
        let x = 500;
        let y = 0;
        while (true) {
            if (y > maxy) {
                return sand.size;
            }
            let hbottom = hash(x, y + 1);
            if (!sand.has(hbottom) && !walls.has(hbottom)) {
                y++;
                continue;
            }
            let hbottomleft = hash(x - 1, y + 1);
            if (!sand.has(hbottomleft) && !walls.has(hbottomleft)) {
                y++;
                x--;
                continue;
            }
            let hbottomright = hash(x + 1, y + 1);
            if (!sand.has(hbottomright) && !walls.has(hbottomright)) {
                y++;
                x++;
                continue;
            }
            sand.add(hash(x, y));
            break;
        }
    }
};
const f1 = ({ walls }) => {
    let t = 0;
    let sand = new Set();
    let maxy = 0;
    walls.forEach((h) => {
        let y = h % 10000;
        if (y > maxy) {
            maxy = y;
        }
    });
    maxy += 1;
    while (true) {
        let x = 500;
        let y = 0;
        while (true) {
            if (y == maxy) {
                sand.add(hash(x, y));
                break;
            }
            let hbottom = hash(x, y + 1);
            if (!sand.has(hbottom) && !walls.has(hbottom)) {
                y++;
                continue;
            }
            let hbottomleft = hash(x - 1, y + 1);
            if (!sand.has(hbottomleft) && !walls.has(hbottomleft)) {
                y++;
                x--;
                continue;
            }
            let hbottomright = hash(x + 1, y + 1);
            if (!sand.has(hbottomright) && !walls.has(hbottomright)) {
                y++;
                x++;
                continue;
            }
            if (x == 500 && y == 0) {
                return sand.size + 1;
            }
            sand.add(hash(x, y));
            break;
        }
    }
};
(0, main_js_1.test)([
    { f: f0, expected: 24 },
    { f: f1, expected: 93 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map