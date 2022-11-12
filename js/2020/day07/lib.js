import { benchmark, test } from "./main.js";

export const format = (lines) => {
  let assocs = {};
  lines.forEach((line) => {
    let words = line.split(" ");
    let name = words[0] + words[1];
    let inside = [];
    for (let i = 4; i < words.length; i += 4) {
      if (words[i] === "no") {
        break;
      }
      let inside_name = words[i + 1] + words[i + 2];
      inside.push([parseInt(words[i]), inside_name]);
    }
    assocs[name] = inside;
  });
  return [assocs, "shinygold"];
};

const can_contain_id = (id, goal_id, assocs) => {
  return (
    id == goal_id ||
    assocs[id]
      .map(([_, child_id]) => can_contain_id(child_id, goal_id, assocs))
      .reduce((a, b) => a || b, false)
  );
};

const f0 = ([assocs, gold_bag_id]) => {
  return (
    Object.keys(assocs)
      .map((id) => can_contain_id(id, gold_bag_id, assocs))
      .reduce((a, b) => a + b) - 1
  );
};

const count_bags = (id, assocs) => {
  return assocs[id]
    .map(
      ([child_count, child_id]) =>
        child_count * (count_bags(child_id, assocs) + 1)
    )
    .reduce((a, b) => a + b, 0);
};

const f1 = ([assocs, gold_bag_id]) => {
  return count_bags(gold_bag_id, assocs);
};

test([
  { f: f0, expected: 4 },
  { f: f1, expected: 32 },
  { f: f1, expected: 126, file: "test_input1.txt" },
]);

benchmark(f0, f1);
