import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  let rules = [];
  let i = 0;
  let words = lines[0].split(" ");
  while (words[1].slice(0, 6) != "ticket") {
    rules.push([
      lines[i].split(":")[0],
      words[1].split("-").map((x) => parseInt(x)),
      words[3].split("-").map((x) => parseInt(x)),
    ]);

    i++;
    words = lines[i].split(" ");
  }

  i++;

  const personnal_ticket = lines[i].split(",").map((x) => parseInt(x));
  i += 2;
  let tickets = [];
  while (i < lines.length) {
    tickets.push(lines[i].split(",").map((x) => parseInt(x)));
    i++;
  }

  return [rules, personnal_ticket, tickets];
};

const follows_rule = (
  value: number,
  [_, [min0, max0], [min1, max1]]
): boolean => {
  return (value >= min0 && value <= max0) || (value >= min1 && value <= max1);
};

const get_invalid_fields = (ticket: number[], rules): number[] => {
  let invalids = [];

  for (let i = 0; i < ticket.length; i++) {
    let is_valid = false;
    for (let rule_index = 0; rule_index < rules.length; rule_index++) {
      if (follows_rule(ticket[i], rules[rule_index])) {
        is_valid = true;
        break;
      }
    }

    if (!is_valid) {
      invalids.push(ticket[i]);
    }
  }

  return invalids;
};

const f0 = ([rules, personnal_ticket, tickets]) => {
  return tickets
    .flatMap((t) => get_invalid_fields(t, rules))
    .reduce((a, b) => a + b, 0);
};

const get_possible_fields_indexes = (value: number, rules): number[] => {
  let possible_fields_indexes = [];

  for (let rule_index = 0; rule_index < rules.length; rule_index++) {
    if (follows_rule(value, rules[rule_index])) {
      possible_fields_indexes.push(rule_index);
    }
  }

  return possible_fields_indexes;
};

const f1 = ([rules, personnal_ticket, tickets]) => {
  tickets = tickets.filter((t) => get_invalid_fields(t, rules).length === 0);
  let field_indexes: Set<number>[] = Array.from(
    { length: rules.length },
    () => {
      let s = new Set<number>();
      for (let i = 0; i < rules.length; i++) {
        s.add(i);
      }
      return s;
    }
  );
  let determined_count = 0;

  for (let ticket_index = 0; ticket_index < tickets.length; ticket_index++) {
    for (let rule_index = 0; rule_index < rules.length; rule_index++) {}
  }

  return rules
    .map(([name, mm0, mm1], i) => {
      if (name.slice(0, 9) === "departure") {
        return personnal_ticket[field_indexes[i].values().next().value];
      } else {
        return 1;
      }
    })
    .reduce((a, b) => a * b, 1);
};

test([
  { f: f0, expected: 71 },
  { f: f1, expected: 1 },
]);

benchmark(f0, f1);
