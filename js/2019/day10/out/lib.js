"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var Point = /** @class */ (function () {
    function Point(x, y) {
        this.x = x;
        this.y = y;
    }
    Point.prototype.wayTo = function (other) {
        return new Point(other.x - this.x, other.y - this.y);
    };
    Object.defineProperty(Point.prototype, "moduleSquared", {
        get: function () {
            return this.x * this.x + this.y * this.y;
        },
        enumerable: false,
        configurable: true
    });
    Point.prototype.equals = function (other) {
        return this.x === other.x && this.y === other.y;
    };
    return Point;
}());
var format = function (lines) {
    return lines.map(function (line) { return Array.from(line).map(function (c) { return c == "#"; }); });
};
exports.format = format;
var is_point_between = function (p0, p1, potential_middle) {
    var dist0 = p0.wayTo(p1);
    var dist1 = p0.wayTo(potential_middle);
    return (dist0.moduleSquared > dist1.moduleSquared &&
        dist0.x * dist1.x >= 0 &&
        dist0.y * dist1.y >= 0 &&
        dist0.x * dist1.y === dist1.x * dist0.y);
};
var get_points = function (grid) {
    var points = [];
    for (var i = 0; i < grid.length; i++) {
        for (var j = 0; j < grid[0].length; j++) {
            if (grid[i][j])
                points.push(new Point(i, j));
        }
    }
    return points;
};
var get_best_score = function (points) {
    var scores = new Array(points.length).fill(0);
    for (var i = 0; i < points.length; i++) {
        var score = 0;
        var p0 = points[i];
        for (var j = 0; j < points.length; j++) {
            if (i == j)
                continue;
            var is_visible = true;
            var p1 = points[j];
            for (var k = 0; k < points.length; k++) {
                if (k == j || k == i)
                    continue;
                if (is_point_between(p0, p1, points[k])) {
                    is_visible = false;
                    break;
                }
            }
            score += is_visible ? 1 : 0;
        }
        scores[i] = score;
    }
    return scores.reduce(function (_a, val, i) {
        var i_max = _a[0], max = _a[1];
        return (val > max ? [i, val] : [i_max, max]);
    }, [-1, 0]);
};
var f0 = function (grid) {
    var points = get_points(grid);
    return get_best_score(points)[1];
};
var get_asteroid_score = function (p) {
    return p.y * 100 + p.x;
};
var angle = function (p0, p1) {
    return Math.atan2(p0.y - p1.y, p0.x - p1.x) * (180 / Math.PI);
};
var sort_points_by_rotation = function (p0, p1, station) {
    if (p0.equals(p1)) {
        return 0;
    }
    else if (
    // aligned case
    are_aligned(p0, p1, station)) {
        return p0.moduleSquared > p1.moduleSquared ? -1 : 1;
    }
    else {
        return angle(p0, station) < angle(p1, station) ? 1 : -1;
    }
};
var are_aligned = function (p0, p1, reference) {
    var diff0 = reference.wayTo(p0);
    var diff1 = reference.wayTo(p1);
    return (diff0.x * diff1.x >= 0 &&
        diff0.y * diff1.y >= 0 &&
        diff0.x * diff1.y === diff1.x * diff0.y);
};
var f1 = function (grid) {
    var points = get_points(grid);
    var station = points.splice(get_best_score(points)[0], 1)[0];
    points.sort(function (p0, p1) { return sort_points_by_rotation(p0, p1, station); });
    var alignements = [];
    var current_align = [points[0]];
    for (var i = 1; i < points.length; i++) {
        if (are_aligned(points[i], points[i - 1], station)) {
            current_align.push(points[i]);
        }
        else {
            alignements.push(current_align);
            current_align = [points[i]];
        }
    }
    alignements.push(current_align);
    var removal_index = 0;
    var layer_index = 0;
    var alignements_index = 0;
    while (removal_index < 200) {
        if (alignements[alignements_index].length > layer_index) {
            console.log(removal_index, alignements[alignements_index][layer_index].y, alignements[alignements_index][layer_index].x);
            removal_index++;
        }
        if (alignements_index === alignements.length - 1) {
            alignements_index = 0;
            layer_index++;
        }
        else {
            alignements_index++;
        }
    }
    return get_asteroid_score(alignements[alignements_index - 1][layer_index]);
};
(0, main_js_1.test)([
    { f: f0, expected: 210 },
    { f: f0, expected: 8, file: "test_input0.txt" },
    { f: f0, expected: 33, file: "test_input1.txt" },
    { f: f0, expected: 35, file: "test_input2.txt" },
    // { f: f1, expected: undefined, file: "test_input0.txt" },
    { f: f1, expected: 802 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map