"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.test = exports.benchmark = void 0;
const fs_1 = require("fs");
const perf_hooks_1 = require("perf_hooks");
const lib_js_1 = require("./lib.js");
const get_lines = (name) => {
    const contents = (0, fs_1.readFileSync)(name, "utf-8");
    return contents.split(/\r?\n/).filter((line) => line.trim() !== "");
};
const read_input = (name) => (0, lib_js_1.format)(get_lines(name));
const benchmark = (f0, f1) => {
    var startTime0 = perf_hooks_1.performance.now();
    var result0 = f0(read_input("input.txt"));
    var endTime0 = perf_hooks_1.performance.now();
    console.log(`f0 -> ${result0}` +
        " ".repeat(40 - result0.toString().length) +
        `in ${endTime0 - startTime0} ms`);
    var startTime1 = perf_hooks_1.performance.now();
    var result1 = f1(read_input("input.txt"));
    var endTime1 = perf_hooks_1.performance.now();
    console.log(`f1 -> ${result1}` +
        " ".repeat(40 - result1.toString().length) +
        `in ${endTime1 - startTime1} ms`);
};
exports.benchmark = benchmark;
const test = (tests_list) => {
    tests_list.forEach((t) => {
        if (!Object.keys(t).includes("file")) {
            t.file = "test_input.txt";
        }
        test_func(t);
    });
};
exports.test = test;
const test_func = ({ f, expected, file }) => {
    let obtained = f(read_input(file));
    if (obtained == expected) {
        console.log(`${f.name} over ${file + " ".repeat(15 - file.length)} - PASSED (${obtained})`);
    }
    else {
        console.log(`${f.name} over ${file + " ".repeat(15 - file.length)} - FAILED : expected ${expected}, got ${obtained}`);
    }
};
//# sourceMappingURL=main.js.map