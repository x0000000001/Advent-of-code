from os import read
import sys
from math import prod
from pprint import pprint


def is_accepted(part, workflows):
    current_flow = "in"

    while True:
        wf = workflows[current_flow]

        for (char, condition, number), next_w in wf["rules"]:
            if (condition == ">" and part[char] > number) or (
                condition == "<" and part[char] < number
            ):
                current_flow = next_w
                break
        else:
            current_flow = wf["default"]

        if current_flow == "A":
            return True
        elif current_flow == "R":
            return False


def f1(input):
    workflows, parts = input
    return sum([sum(part.values()) for part in parts if is_accepted(part, workflows)])


def pass_range(part_ranges, workflow):
    results = []
    to_scan = [part_ranges]

    for (char, condition, number), next_w in workflow["rules"]:
        new_scan = []

        for pr in to_scan:
            start, end = pr[char]

            if condition == ">" and number < end:
                pr_end = pr.copy()
                pr_end[char] = (max(number + 1, start), end)
                results.append((pr_end, next_w))

            if condition == ">" and number >= start:
                pr_start = pr.copy()
                pr_start[char] = (start, min(number, end))
                new_scan.append(pr_start)

            if condition == "<" and number <= end:
                pr_end = pr.copy()
                pr_end[char] = (max(number, start), end)
                new_scan.append(pr_end)

            if condition == "<" and number > start:
                pr_start = pr.copy()
                pr_start[char] = (start, min(number - 1, end))
                results.append((pr_start, next_w))

        to_scan = new_scan

    for pr in to_scan:
        results.append((pr, workflow["default"]))

    return results


def accepted_ranges(init_range, workflows):
    ranges = [(init_range, "in")]
    accepted = []

    while ranges:
        part_ranges, name = ranges.pop()
        results = pass_range(part_ranges, workflows[name])

        for pr, next_name in results:
            if next_name == "A":
                accepted.append(pr)
            elif next_name == "R":
                pass
            else:
                ranges.append((pr, next_name))

    return accepted


def f2(input):
    workflows, _ = input
    accepted = accepted_ranges(
        {
            "a": (1, 4000),
            "x": (1, 4000),
            "s": (1, 4000),
            "m": (1, 4000),
        },
        workflows,
    )

    return sum(
        prod(end - start + 1 for (start, end) in range.values()) for range in accepted
    )


def parse(s):
    blocks = s.split("\n\n")
    workflows = {}

    for line in blocks[0].splitlines():
        words = line.split("{")
        name = words[0]
        rules = []
        rules_strings = words[1][:-1].split(",")
        default = rules_strings.pop()

        for rule in rules_strings:
            w1 = rule.split(":")
            number = int(w1[0][2:])
            char = w1[0][0]
            condition = w1[0][1]
            rules.append(((char, condition, number), w1[1]))

        workflows[name] = {"rules": rules, "default": default}

    parts = []

    for line in blocks[1].splitlines():
        part = {}

        for s in line[1:-1].split(","):
            part[s[0]] = int(s[2:])

        parts.append(part)

    return workflows, parts


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
input = parse(s)
print(f2(input))
