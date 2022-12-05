"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.format = void 0;
var main_js_1 = require("./main.js");
var format = function (lines) {
    return lines[0].split(" ").map(function (x) { return parseInt(x); });
};
exports.format = format;
var metadata_sum = function (input, index) {
    if (index === void 0) { index = 0; }
    var sum = 0;
    var child_count = input[index];
    var metadata_count = input[index + 1];
    index += 2;
    for (var i = 0; i < child_count; i++) {
        var _a = metadata_sum(input, index), partial_sum = _a[0], new_index = _a[1];
        index = new_index;
        sum += partial_sum;
    }
    for (var _ = 0; _ < metadata_count; _++) {
        sum += input[index];
        index++;
    }
    return [sum, index];
};
var f0 = function (input) {
    return metadata_sum(input)[0];
};
var node_value = function (input, index) {
    if (index === void 0) { index = 0; }
    var sum = 0;
    var child_count = input[index];
    var metadata_count = input[index + 1];
    index += 2;
    var childs_sums = new Array(child_count).fill(null);
    for (var i = 0; i < child_count; i++) {
        var _a = node_value(input, index), partial_sum = _a[0], new_index = _a[1];
        index = new_index;
        childs_sums[i] = partial_sum;
    }
    if (child_count === 0) {
        for (var _ = 0; _ < metadata_count; _++) {
            sum += input[index];
            index++;
        }
    }
    else {
        for (var _ = 0; _ < metadata_count; _++) {
            var pointer = input[index];
            if (pointer > 0 && pointer <= child_count) {
                sum += childs_sums[pointer - 1];
            }
            index++;
        }
    }
    return [sum, index];
};
var f1 = function (input) {
    return node_value(input)[0];
};
(0, main_js_1.test)([
    { f: f0, expected: 138 },
    { f: f1, expected: 66 },
]);
(0, main_js_1.benchmark)(f0, f1);
//# sourceMappingURL=lib.js.map