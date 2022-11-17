import { readFileSync } from "fs";
import { performance } from "perf_hooks";
import { format } from "./lib.js";

const get_lines = (name) => {
  const contents = readFileSync(name, "utf-8");
  return contents.split(/\r?\n/).filter((line) => line.trim() !== "");
};

const read_input = (name) => format(get_lines(name));

export const benchmark = (f0, f1) => {
  var startTime0 = performance.now();
  var result0 = f0(read_input("input.txt"));
  var endTime0 = performance.now();
  console.log(
    `f0 -> ${result0}` +
      " ".repeat(40 - result0.toString().length) +
      `in ${endTime0 - startTime0} ms`
  );

  var startTime1 = performance.now();
  var result1 = f1(read_input("input.txt"));
  var endTime1 = performance.now();
  console.log(
    `f1 -> ${result1}` +
      " ".repeat(40 - result1.toString().length) +
      `in ${endTime1 - startTime1} ms`
  );
};

export const test = (tests_list) => {
  tests_list.forEach((t) => {
    if (!Object.keys(t).includes("file")) {
      t.file = "test_input.txt";
    }
    test_func(t);
  });
};

const test_func = ({ f, expected, file }) => {
  let obtained = f(read_input(file));
  if (obtained == expected) {
    console.log(
      `${f.name} over ${
        file + " ".repeat(15 - file.length)
      } - PASSED (${obtained})`
    );
  } else {
    console.log(
      `${f.name} over ${
        file + " ".repeat(15 - file.length)
      } - FAILED : expected ${expected}, got ${obtained}`
    );
  }
};
