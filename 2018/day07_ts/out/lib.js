"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    return lines.map(function (l) {
        var words = l.split(" ");
        return [words[1], words[7]];
    });
};
exports.format = format;
var get_relations = function (input) {
    var obligations = {};
    input.forEach(function (_a) {
        var x = _a[0], y = _a[1];
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
var f0 = function (input) {
    var relations = get_relations(input);
    var res = "";
    while (relations.length > 0) {
        var candidates = [];
        var candidates_ids = {};
        for (var index = 0; index < relations.length; index++) {
            var _a = relations[index], key_1 = _a[0], obl = _a[1];
            if (obl.length === 0) {
                candidates.push(key_1);
                candidates_ids[key_1] = index;
            }
        }
        candidates.sort();
        var key = candidates.shift();
        relations.splice(candidates_ids[key], 1);
        res += key;
        for (var i = 0; i < relations.length; i++) {
            var remove_index = relations[i][1].indexOf(key);
            if (remove_index != -1) {
                relations[i][1].splice(remove_index, 1);
            }
        }
    }
    return res;
};
var time_for_step = function (c, offset) {
    return c.charCodeAt(0) - 64 + offset;
};
var compute_time = function (relations, time_offset, workers_count) {
    var workers_tasks = new Array(workers_count).fill(null);
    var time = 0;
    while (relations.length > 0 ||
        workers_tasks.map(function (x) { return x !== null; }).reduce(function (a, b) { return a || b; }, false)) {
        var candidates = [];
        for (var index = 0; index < relations.length; index++) {
            var _a = relations[index], key = _a[0], obl = _a[1];
            if (obl.length === 0) {
                candidates.push(key);
            }
        }
        candidates.sort();
        for (var index = 0; index < workers_count; index++) {
            if (candidates.length === 0) {
                break;
            }
            if (workers_tasks[index] === null) {
                var key = candidates.shift();
                var id = -1;
                for (var i = 0; i < relations.length; i++) {
                    var element = relations[i];
                    if (element[0] == key) {
                        id = i;
                        break;
                    }
                }
                relations.splice(id, 1);
                workers_tasks[index] = [key, time_for_step(key, time_offset)];
            }
        }
        for (var index = 0; index < workers_count; index++) {
            if (workers_tasks[index] === null) {
                continue;
            }
            workers_tasks[index][1]--;
            if (workers_tasks[index][1] === 0) {
                var key = workers_tasks[index][0];
                for (var i = 0; i < relations.length; i++) {
                    var remove_index = relations[i][1].indexOf(key);
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
var f1 = function (input) {
    return compute_time(get_relations(input), 60, 5);
};
(0, main_js_1.test)([
    { f: f0, expected: "CABDFE" },
    { f: function (x) { return compute_time(get_relations(x), 0, 2); }, expected: 15 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map