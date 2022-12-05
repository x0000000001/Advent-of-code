"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    return parseInt(lines[0]);
};
exports.format = format;
var power_level = function (x, y, serial) {
    x++;
    y++;
    var rackid = x + 10;
    var p = rackid * y + serial;
    p *= rackid;
    var s = p.toString();
    return (s.length >= 3 ? parseInt(s.charAt(s.length - 3)) : 0) - 5;
};
var get_grid = function (serial) {
    var grid = Array.from({ length: 300 }, function () { return new Array(300).fill(0); });
    for (var i = 0; i < 300; i++) {
        for (var j = 0; j < 300; j++) {
            grid[i][j] = power_level(i, j, serial);
        }
    }
    return grid;
};
var max_nxn_square = function (grid, n) {
    var _a, _b;
    var width = grid.length;
    var _c = [0, 0], xmax = _c[0], ymax = _c[1];
    var max_score = 0;
    var column_sums = Array.from({ length: width - n }, function () {
        return new Array(width).fill(0);
    });
    for (var j = 0; j < width; j++) {
        var subsum = 0;
        for (var k = 0; k < n; k++) {
            subsum += grid[k][j];
        }
        for (var i = 0; i < width - n; i++) {
            column_sums[i][j] = subsum;
            subsum += grid[i + n][j] - grid[i][j];
        }
        column_sums[width - n - 1][j] = subsum;
    }
    for (var i = 0; i < width - n; i++) {
        var subsum = 0;
        for (var k = 0; k < n; k++) {
            subsum += column_sums[i][k];
        }
        for (var j = 0; j < width - n; j++) {
            if (subsum > max_score)
                _a = [i, j, subsum], xmax = _a[0], ymax = _a[1], max_score = _a[2];
            subsum += column_sums[i][j + n] - column_sums[i][j];
        }
        if (subsum > max_score)
            _b = [i, width - n - 1, subsum], xmax = _b[0], ymax = _b[1], max_score = _b[2];
    }
    return [xmax + 1, ymax + 1, max_score];
};
var f0 = function (input) {
    return max_nxn_square(get_grid(input), 3).slice(0, 2);
};
var max_square = function (grid) {
    var _a;
    var _b = [0, 0, 0], xmax = _b[0], ymax = _b[1], length_max = _b[2];
    var max_score = 0;
    for (var i = 1; i < grid.length; i++) {
        var _c = max_nxn_square(grid, i), x = _c[0], y = _c[1], score = _c[2];
        if (score > max_score) {
            max_score = score;
            _a = [x, y, i], xmax = _a[0], ymax = _a[1], length_max = _a[2];
        }
    }
    return [xmax, ymax, length_max];
};
var f1 = function (input) {
    return max_square(get_grid(input));
};
(0, main_js_1.test)([
    { f: function (x) { return power_level(2, 4, x); }, expected: 4 },
    {
        f: function (x) { return f1(x).toString(); },
        expected: "90,269,16",
        file: "test_input1.txt",
    },
    {
        f: function (x) { return f1(x).toString(); },
        expected: "232,251,12",
        file: "test_input2.txt",
    },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map