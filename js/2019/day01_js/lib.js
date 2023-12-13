import { benchmark, test } from "./main.js";

export const format = (lines) => {
  return lines.map((x) => parseInt(x));
};

const f0 = (input) => {
  return input.reduce((acc, b) => acc + (Math.floor(b / 3) - 2), 0);
};

const fuel_for_mass = (mass) => {
  const added_fuel = Math.floor(mass / 3) - 2;

  if (added_fuel <= 0) {
    return 0;
  }

  return added_fuel + fuel_for_mass(added_fuel);
};

const f1 = (input) => {
  return input.reduce((acc, b) => acc + fuel_for_mass(b), 0);
};

test([]);

benchmark(f0, f1);
