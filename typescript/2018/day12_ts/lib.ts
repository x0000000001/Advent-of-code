import { benchmark, test } from "./main.js";

type Data = {
  plants: Set<number>;
  rules: boolean[][];
  min_index: number;
  max_index: number;
};

export const format = (lines: string[]): Data => {
  const string_to_bool_array = (s) => Array.from(s).map((c) => c === "#");
  let initial_state_array = string_to_bool_array(lines[0].split(" ")[2]);
  let initial_state = new Set<number>();
  for (const [i, b] of initial_state_array.entries()) {
    if (b) {
      initial_state.add(i);
    }
  }

  let rules = [];
  for (let i = 1; i < lines.length; i++) {
    let temp = lines[i].split(" => ");
    if (temp[1] == "#") {
      rules.push(string_to_bool_array(temp[0]));
    }
  }

  return {
    plants: initial_state,
    rules,
    min_index: -5,
    max_index: initial_state_array.length + 5,
  };
};

const iterate = (data: Data): void => {
  let new_plants = new Set<number>();
  let new_min_index = data.min_index;
  let new_max_index = data.max_index;

  for (let i = data.min_index; i < data.max_index - 2; i++) {
    for (let j = 0; j < data.rules.length; j++) {
      let follows_rule = true;
      for (let k = 0; k < 5; k++) {
        if (data.rules[j][k] !== data.plants.has(i + k - 2)) {
          follows_rule = false;
          break;
        }
      }
      if (follows_rule) {
        new_plants.add(i);

        if (i > new_max_index - 5) {
          new_max_index = i + 5;
        } else if (i < new_min_index + 5) {
          new_min_index = i - 5;
        }

        break;
      }
    }
  }

  data.plants = new_plants;
  data.min_index = new_min_index;
  data.max_index = new_max_index;
};

const compute_score = ({ plants }: Data): number => {
  return [...plants].reduce((a, b) => a + b, 0);
};

const print_data = (data: Data): void => {
  let s = "";
  for (let index = data.min_index; index < data.max_index; index++) {
    if (data.plants.has(index)) {
      s += "#";
    } else {
      s += ".";
    }
  }
  console.log(s);
};

const f0 = (data: Data): number => {
  // print_data(data);
  for (let i = 0; i < 20; i++) {
    iterate(data);
    // console.log(i);
    // print_data(data);
    // console.log(compute_score(data));
  }

  return compute_score(data);
};

//3399999996471 too high
const f1 = (data: Data): number => {
  let score = compute_score(data);
  let last_difference = null;
  let same_difference_count = 0;
  for (let i = 0; i < 50000000000; i++) {
    iterate(data);
    let temp_score = compute_score(data);
    if (temp_score - score == last_difference) {
      same_difference_count += 1;
    } else {
      same_difference_count = 0;
    }
    if (same_difference_count > 100) {
      score += last_difference * (50000000000 - i);
      console.log(last_difference);
      break;
    }
    last_difference = temp_score - score;
    score = temp_score;
  }

  return score;
};

test([{ f: f0, expected: 325 }]);

benchmark(f0, f1);
