"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    var compute_line = function (line) {
        var words = line.split(" velocity=<");
        var _a = words[1]
            .substring(0, words[1].length - 1)
            .split(",")
            .map(function (x) { return parseInt(x); }), vx = _a[0], vy = _a[1];
        var temp0 = words[0].split("<");
        var _b = temp0[1]
            .substring(0, temp0[1].length - 1)
            .split(",")
            .map(function (x) { return parseInt(x); }), x = _b[0], y = _b[1];
        return [x, y, vx, vy];
    };
    return lines.map(compute_line);
};
exports.format = format;
var iterate = function (input) {
    for (var i = 0; i < input.length; i++) {
        input[i][0] += input[i][2];
        input[i][1] += input[i][3];
    }
};
var print_stars = function (input, step) {
    var _a = input
        .map(function (_a) {
        var x = _a[0], y = _a[1], vx = _a[2], vy = _a[3];
        return [x, y];
    })
        .reduce(function (_a, _b) {
        var minx = _a[0], miny = _a[1], maxx = _a[2], maxy = _a[3];
        var x = _b[0], y = _b[1];
        return [
            Math.min(x, minx),
            Math.min(y, miny),
            Math.max(x, maxx),
            Math.max(y, maxy),
        ];
    }, [Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER, 0, 0]), minx = _a[0], miny = _a[1], maxx = _a[2], maxy = _a[3];
    var _b = [maxx - minx + 1, maxy - miny + 1], width = _b[0], height = _b[1];
    if (width > 200 || height > 200) {
        return;
    }
    var grid = Array.from(Array(width), function () { return new Array(height).fill(-1); });
    for (var i = 0; i < input.length; i++) {
        grid[input[i][0] - minx][input[i][1] - miny] = i;
    }
    for (var j = 0; j < height; j++) {
        var line = "";
        for (var i = 0; i < width; i++) {
            line += grid[i][j] == -1 ? ". " : "# ";
        }
        console.log(line);
    }
    console.log("TIME : ".concat(step, " "));
};
var print_animation = function (input, time_limit) {
    var i = 0;
    while (i < time_limit) {
        iterate(input);
        print_stars(input, i);
        i++;
    }
    console.log("FINISHED PRINTING");
};
var f0 = function (input) {
    print_animation(input, 100000);
    return 0;
};
var f1 = function (input) {
    print_animation(input, 100000);
    return 0;
};
(0, main_js_1.test)([{ f: function (x) { return print_animation(x, 3); }, expected: 0 }]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map