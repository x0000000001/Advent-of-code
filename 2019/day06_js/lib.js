import { benchmark, test } from "./main.js";

class OrbitsTree {
  constructor(value, children = []) {
    this.value = value;
    this.children = children;
  }

  insert(parent_name, child_name) {
    // reroot case
    if (child_name == this.value) {
      const child_temp = this.children;
      this.value = parent_name;
      this.children = [new OrbitsTree(child_name)];
      this.children[0].children = child_temp;
      return true;
    }

    if (parent_name == this.value) {
      this.children.push(new OrbitsTree(child_name));
      return true;
    } else {
      for (const index in this.children) {
        if (this.children[index].insert(parent_name, child_name)) {
          return true;
        }
      }
    }

    return false;
  }

  print(indent = 0) {
    console.log("   ".repeat(indent) + "|" + this.value);
    this.children.forEach((c) => c.print(indent + 1));
  }

  find_path_to = (goal) => {
    if (this.value == goal) {
      return [];
    }

    for (let index = 0; index < this.children.length; index++) {
      let path = this.children[index].find_path_to(goal);
      if (path !== null) {
        path.push(this.value);
        return path;
      }
    }

    return null;
  };
}

const build_tree = (orbits) => {
  let added_count = 0;
  let added = new Array(orbits.length).fill(false);
  let tree = new OrbitsTree(orbits[0][0], [new OrbitsTree(orbits[0][1])]);
  added_count++;
  added[0] = true;
  while (added_count != orbits.length) {
    for (let index = 1; index < orbits.length; index++) {
      if (added[index]) {
        continue;
      }
      if (tree.insert(orbits[index][0], orbits[index][1])) {
        added_count++;
        added[index] = true;
      }
    }
  }
  return tree;
};

export const format = (lines) => {
  return lines.map((l) => l.split(")"));
};

const count_all_orbits = (orbitsTree, depth = 1) => {
  return orbitsTree.children
    .map((t) => count_all_orbits(t, depth + 1))
    .reduce((a, b) => a + b + depth, 0);
};

const f0 = (orbits) => {
  return count_all_orbits(build_tree(orbits));
};

const f1 = (orbits) => {
  const t = build_tree(orbits);
  let you_path = t.find_path_to("YOU");
  let santa_path = t.find_path_to("SAN");
  let i = -1;
  while (you_path[you_path.length + i] == santa_path[santa_path.length + i]) {
    i--;
  }
  return you_path.length + i + santa_path.length + i + 2;
};

test([
  { f: f0, expected: 42 },
  { f: f1, expected: 4, file: "test_input1.txt" },
]);

benchmark(f0, f1);
