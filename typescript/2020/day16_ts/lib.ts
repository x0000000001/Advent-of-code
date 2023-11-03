import { benchmark, test } from "./main.js";

export const format = (lines: string[]) => {
  let rules = [];
  let i = 0;
  while (!lines[i].startsWith("your")) {
    let words = lines[i].split(": ");
    let temp = words[1].split(" ");

    rules.push([
      words[0],
      temp[0].split("-").map((x) => parseInt(x)),
      temp[2].split("-").map((x) => parseInt(x)),
    ]);

    i++;
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
  [_, [min0, max0], [min1, max1]]: [string, [number, number], [number, number]]
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

const f1 = ([fields, personnal_ticket, tickets]) => {
  tickets = tickets.filter((t) => get_invalid_fields(t, fields).length === 0);
  let possibilities: [number, Set<number>][] = Array.from({
    length: fields.length,
  }).map((_, original_index) => [
    original_index,
    new Set(
      Array.from({ length: fields.length })
        .map((_, candidate_index) => {
          for (
            let ticket_index = 0;
            ticket_index < tickets.length;
            ticket_index++
          ) {
            let t = tickets[ticket_index][candidate_index];
            let f = fields[original_index];
            if (
              !follows_rule(
                tickets[ticket_index][candidate_index],
                fields[original_index]
              )
            ) {
              return null;
            }
          }
          return candidate_index;
        })
        .filter((x) => x !== null)
    ),
  ]);

  let used = new Set();
  possibilities = possibilities
    .sort(([indexa, a], [indexb, b]) => a.size - b.size)
    .filter(([_, s]) => s.size != 0);
  return possibilities
    .map(([original_index, possibilities]) => {
      const used_index: number = [...possibilities].filter(
        (x) => !used.has(x)
      )[0];
      // let y = [...possibilities].filter((x) => !used.has(x));
      // console.log(used_index, y, original_index);
      used.add(used_index);

      if (fields[original_index][0].startsWith("departure")) {
        return personnal_ticket[used_index];
      } else {
        return 1;
      }
    })
    .reduce((a, b) => a * b, 1);
};

// 1970155307291 too low
// 4805792588689 too high

test([
  { f: f0, expected: 71 },
  // { f: f1, expected: 1 },
]);

benchmark(f0, f1);
