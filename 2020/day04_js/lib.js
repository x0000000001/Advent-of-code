import { benchmark, test } from "./main.js";

export const format = (lines) => {
  let passeports = [];
  let p = {};

  lines.forEach((line) => {
    if (line.trim() == "") {
      if (Object.keys(p).length != 0) {
        passeports.push(p);
      }
      p = {};
    } else {
      let attributes = line.split(" ").map((word) => word.split(":"));
      attributes.forEach(([name, value]) => {
        p[name] = value;
      });
    }
  });

  if (Object.keys(p).length != 0) {
    passeports.push(p);
  }

  return passeports;
};

var f0MandatoryFields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const is_valid0 = (passeport) => {
  return f0MandatoryFields
    .map((key) => Object.keys(passeport).includes(key))
    .reduce((a, b) => a * b);
};

const f0 = (input) => {
  return input.filter(is_valid0).length;
};

global.is_byr_valid = (byr) => {
  return /^[0-9]{4}$/.test(byr) && byr >= 1920 && byr <= 2002;
};

global.is_iyr_valid = (iyr) => {
  return /^[0-9]{4}$/.test(iyr) && iyr >= 2010 && iyr <= 2020;
};

global.is_eyr_valid = (eyr) => {
  return /^[0-9]{4}$/.test(eyr) && eyr >= 2020 && eyr <= 2030;
};

global.is_hgt_valid = (hgt) => {
  let unit = hgt.substring(hgt.length - 2);
  let value = parseInt(hgt.substring(0, hgt.length - 2));

  switch (unit) {
    case "cm":
      return value >= 150 && value <= 193;
    case "in":
      return value >= 59 && value <= 76;
    default:
      return false;
  }
};

global.is_hcl_valid = (hcl) => {
  return /^#[a-f0-9]{6}$/.test(hcl);
};

global.is_ecl_valid = (ecl) => {
  return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(ecl);
};

global.is_pid_valid = (pid) => {
  return /^[0-9]{9}$/.test(pid);
};

const is_valid1 = (passeport) => {
  return f0MandatoryFields
    .map(
      (key) =>
        Object.keys(passeport).includes(key) &&
        global[`is_${key}_valid`](passeport[key])
    )
    .reduce((a, b) => a * b);
};

const f1 = (input) => {
  return input.filter(is_valid1).length;
};

test([
  { f: f0, expected: 2 },
  { f: f1, expected: 0, file: "invalid.txt" },
  { f: f1, expected: 4, file: "valid.txt" },
]);

benchmark(f0, f1);
