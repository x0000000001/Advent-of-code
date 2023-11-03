import { type } from "os";
import { benchmark, test } from "./main.js";

type Op = "times" | "plus";

type OpTree =
  | { tree_type: "val"; val: number }
  | { tree_type: "node"; op: Op; left: OpTree; right: OpTree };

const get_tree = (
  s: string,
  get_first_op_index: (s: string) => number
): OpTree => {
  s = s.trim();
  if (s.charAt(0) == "(" && s.charAt(s.length - 1) == ")") {
    let can_remove_par = true;
    let par_count = 0;

    for (let index = 1; index < s.length - 1; index++) {
      switch (s.charAt(index)) {
        case "(":
          par_count++;
          break;
        case ")":
          par_count--;
          break;
      }

      if (par_count < 0) {
        can_remove_par = false;
        break;
      }
    }

    if (par_count != 0) can_remove_par = false;

    if (can_remove_par) s = s.substring(1, s.length - 1);
  }

  if (/^\d+$/i.test(s)) {
    return { tree_type: "val", val: parseInt(s) };
  }

  let op: Op = null;
  let op_index = get_first_op_index(s);
  switch (s.charAt(op_index)) {
    case "+":
      op = "plus";
      break;
    case "*":
      op = "times";
      break;
  }

  return {
    tree_type: "node",
    op,
    left: get_tree(s.substring(0, op_index), get_first_op_index),
    right: get_tree(s.substring(op_index + 1), get_first_op_index),
  };
};

const get_first_index0 = (s: string): number => {
  let i = s.length - 1;
  let parentheses_count = 0;
  for (const c of Array.from(s).reverse()) {
    switch (c) {
      case "(":
        parentheses_count++;
        break;
      case ")":
        parentheses_count--;
        break;
    }

    if (parentheses_count === 0) {
      if (c == "+" || c == "*") {
        return i;
      }
    }
    i--;
  }
};

export const format = (lines: string[]) => {
  return lines;
};

const eval_tree = (t: OpTree) => {
  if (t.tree_type == "val") {
    return t.val;
  } else {
    switch (t.op) {
      case "plus":
        return eval_tree(t.left) + eval_tree(t.right);
      case "times":
        return eval_tree(t.left) * eval_tree(t.right);
    }
  }
};

const f0 = (input) => {
  return input
    .map((l) => get_tree(l, get_first_index0))
    .map(eval_tree)
    .reduce((a, b) => a + b, 0);
};

const get_first_index1 = (s: string): number => {
  let i = s.length - 1;
  let parentheses_count = 0;
  let possible_indexes = [];
  for (const c of Array.from(s).reverse()) {
    switch (c) {
      case "(":
        parentheses_count++;
        break;
      case ")":
        parentheses_count--;
        break;
    }

    if (parentheses_count === 0) {
      if (c == "+" || c == "*") {
        possible_indexes.push([c, i]);
      }
    }
    i--;
  }
  possible_indexes = possible_indexes.reverse();
  for (let index = 0; index < possible_indexes.length; index++) {
    if (possible_indexes[index][0] === "*") {
      return possible_indexes[index][1];
    }
  }

  return possible_indexes[0][1];
};

const f1 = (input) => {
  return input
    .map((l) => get_tree(l, get_first_index1))
    .map(eval_tree)
    .reduce((a, b) => a + b, 0);
};

test([
  { f: f0, expected: 13632 },
  { f: f1, expected: 23340 },
]);

benchmark(f0, f1);
