import { benchmark, test } from "./main.js";

type InputType = [boolean[], [boolean[], boolean][]];

export const format = (lines: string[]) => {
  const string_to_bool_array = (s) => Array.from(s).map((c) => c === "#");
  let initial_state = string_to_bool_array(lines[0].split(" ")[2]);
  initial_state.splice(0, 0, false, false, false, false, false);
  initial_state.splice(
    initial_state.length,
    0,
    false,
    false,
    false,
    false,
    false,
    false
  );
  let rules = [];
  for (let i = 1; i < lines.length; i++) {
    let temp = lines[i].split(" => ");
    rules.push(string_to_bool_array(temp[0]), temp[1] == "#");
  }
  return { plants: initial_state, rules, offset: -5 };
};

const iterate = (data) => {
  let add_at_beginning = false;
  let add_at_end = false;
  let new_plants = new Array(data.plants.length).fill(false);
  for (let i = 2; i < data.plants.length - 2; i++) {
    for (let j = 0; j < data.rules.length; j++) {
      let follows_rule = true;
      for (let k = 0; k < 5; k++) {
        if (data.plants[i + k - 2] !== data.rules[j][k]) {
          follows_rule = false;
          break;
        }
      }
      if (follows_rule) {
        new_plants[i] = data.rules[j][1];

        if (i < 2) {
          add_at_beginning = true;
        } else if (i > data.plants.length - 2) {
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
    data.plants.splice(
      data.plants.length,
      0,
      false,
      false,
      false,
      false,
      false
    );
  }
};

const compute_score = ({ plants, offset }) => {
  return plants.map((x, i) => (x ? i + offset : 0)).reduce((a, b) => a + b, 0);
};

const print_data = (data) => {
  console.log(
    data.plants.map((x) => (x ? "#" : ".")).reduce((a, b) => a + b, "")
  );
  console.log(data.offset);
};

const f0 = (data) => {
  for (let _ = 0; _ < 20; _++) {
    print_data(data);
    iterate(data);
  }

  return compute_score(data);
};

const f1 = (input) => {
  return 0;
};

test([
  { f: f0, expected: 325 },
  { f: f1, expected: 0 },
]);

benchmark(f0, f1);
