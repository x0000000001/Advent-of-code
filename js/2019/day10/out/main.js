"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.test = exports.benchmark = void 0;
var fs_1 = require("fs");
var perf_hooks_1 = require("perf_hooks");
var lib_js_1 = require("./lib.js");
var get_lines = function (name) {
    var contents = (0, fs_1.readFileSync)(name, "utf-8");
    return contents.split(/\r?\n/).filter(function (line) { return line.trim() !== ""; });
};
var read_input = function (name) { return (0, lib_js_1.format)(get_lines(name)); };
var benchmark = function (f0, f1) {
    var startTime0 = perf_hooks_1.performance.now();
    var result0 = f0(read_input("input.txt"));
    var endTime0 = perf_hooks_1.performance.now();
    console.log("f0 -> ".concat(result0) +
        " ".repeat(40 - result0.toString().length) +
        "in ".concat(endTime0 - startTime0, " ms"));
    var startTime1 = perf_hooks_1.performance.now();
    var result1 = f1(read_input("input.txt"));
    var endTime1 = perf_hooks_1.performance.now();
    console.log("f1 -> ".concat(result1) +
        " ".repeat(40 - result1.toString().length) +
        "in ".concat(endTime1 - startTime1, " ms"));
};
exports.benchmark = benchmark;
var test = function (tests_list) {
    tests_list.forEach(function (t) {
        if (!Object.keys(t).includes("file")) {
            t.file = "test_input.txt";
        }
        test_func(t);
    });
};
exports.test = test;
var test_func = function (_a) {
    var f = _a.f, expected = _a.expected, file = _a.file;
    var obtained = f(read_input(file));
    if (obtained == expected) {
        console.log("".concat(f.name, " over ").concat(file + " ".repeat(15 - file.length), " - PASSED (").concat(obtained, ")"));
    }
    else {
        console.log("".concat(f.name, " over ").concat(file + " ".repeat(15 - file.length), " - FAILED : expected ").concat(expected, ", got ").concat(obtained));
    }
};
//# sourceMappingURL=main.js.map