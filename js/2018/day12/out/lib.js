"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    var string_to_bool_array = function (s) { return Array.from(s).map(function (c) { return c === "#"; }); };
    var initial_state = string_to_bool_array(lines[0].split(" ")[2]);
    initial_state.splice(0, 0, false, false, false, false, false);
    initial_state.splice(initial_state.length, 0, false, false, false, false, false, false);
    var rules = [];
    for (var i = 1; i < lines.length; i++) {
        var temp = lines[i].split(" => ");
        rules.push(string_to_bool_array(temp[0]), temp[1] == "#");
    }
    return { plants: initial_state, rules: rules, offset: -5 };
};
exports.format = format;
var iterate = function (data) {
    var add_at_beginning = false;
    var add_at_end = false;
    var new_plants = new Array(data.plants.length).fill(false);
    for (var i = 2; i < data.plants.length - 2; i++) {
        for (var j = 0; j < data.rules.length; j++) {
            var follows_rule = true;
            for (var k = 0; k < 5; k++) {
                if (data.plants[i + k - 2] !== data.rules[j][k]) {
                    follows_rule = false;
                    break;
                }
            }
            if (follows_rule) {
                new_plants[i] = data.rules[j][1];
                if (i < 2) {
                    add_at_beginning = true;
                }
                else if (i > data.plants.length - 2) {
                    add_at_end = true;
                }
            }
        }
    }
    data.plants = new_plants;
    if (add_at_beginning) {
        data.plants.splice(0, 0, false, false, false, false, false);
        data.offset += 5;
    }
    if (add_at_end) {
        data.plants.splice(data.plants.length, 0, false, false, false, false, false);
    }
};
var compute_score = function (_a) {
    var plants = _a.plants, offset = _a.offset;
    return plants.map(function (x, i) { return (x ? i + offset : 0); }).reduce(function (a, b) { return a + b; }, 0);
};
var print_data = function (data) {
    console.log(data.plants.map(function (x) { return (x ? "#" : "."); }).reduce(function (a, b) { return a + b; }, ""));
    console.log(data.offset);
};
var f0 = function (data) {
    for (var _ = 0; _ < 20; _++) {
        print_data(data);
        iterate(data);
    }
    return compute_score(data);
};
var f1 = function (input) {
    return 0;
};
(0, main_js_1.test)([
    { f: f0, expected: 325 },
    { f: f1, expected: 0 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map