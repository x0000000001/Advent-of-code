import { readFileSync } from "fs";
import { performance } from "perf_hooks";
import { format } from "./lib.js";

const get_lines = (name) => {
  const contents = readFileSync(name, "utf-8");
  return contents.split(/\r?\n/).filter((line) => line.trim() !== "");
};

const read_input = (name) => format(get_lines("input.txt"));
const read_test_input = (name) => format(get_lines("test_input.txt"));

export const benchmark = (f0, f1) => {
  var startTime0 = performance.now();
  var result0 = f0(read_input());
  var endTime0 = performance.now();
  console.log(
    `f0 -> ${result0}` +
      " ".repeat(20 - result0.toString().length) +
      `in ${endTime0 - startTime0} ms`
  );

  var startTime1 = performance.now();
  var result1 = f1(read_input());
  var endTime1 = performance.now();
  console.log(
    `f1 -> ${result1}` +
      " ".repeat(20 - result1.toString().length) +
      `in ${endTime1 - startTime1} ms`
  );
};

export const test = (f0, f1, expected0, expected1) => {
  var obtained0 = f0(read_test_input());
  if (obtained0 == expected0) {
    console.log(`f0 PASSED (${obtained0})`);
  } else {
    console.log(`f0 FAILED : expected ${expected0}, got ${obtained0}`);
  }
  var obtained1 = f1(read_test_input());
  if (obtained1 == expected1) {
    console.log(`f1 PASSED (${obtained1})`);
  } else {
    console.log(`f1 FAILED : expected ${expected1}, got ${obtained1}`);
  }
};
