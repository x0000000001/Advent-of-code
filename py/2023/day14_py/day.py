import sys
from pprint import pprint


def weight(base, count):
    return (base - count + 1) * count + (count - 1) * count // 2


def apply_gravity(input, F):
    n = len(input)

    for j in range(n):
        base = 0
        count = 0
        for i in range(n):
            new_i, new_j = F(i, j)
            c = input[new_i][new_j]

            if c == "#":
                if count > 0:
                    for k in range(base, base + count):
                        kp, kj = F(k, j)
                        input[kp][kj] = "O"
                    for k in range(base + count, i):
                        kp, kj = F(k, j)
                        input[kp][kj] = "."
                base = i + 1
                count = 0
            elif c == "O":
                count += 1

        if count > 0:
            for k in range(base, base + count):
                kp, kj = F(k, j)
                input[kp][kj] = "O"
            for k in range(base + count, n):
                kp, kj = F(k, j)
                input[kp][kj] = "."


def cycle(input):
    n = len(input)
    apply_gravity(input, lambda i, j: (i, j))
    apply_gravity(input, lambda i, j: (j, i))
    apply_gravity(input, lambda i, j: (n - 1 - i, j))
    apply_gravity(input, lambda i, j: (j, n - 1 - i))


def hash(input):
    h = 0
    i = 0

    for line in input:
        for c in line:
            if c == "O":
                h += 1 << i
            i += 1

    return h


def load(input):
    n = len(input)
    load = 0
    for i in range(n):
        load += (n - i) * len([j for j in range(n) if input[i][j] == "O"])

    return load


def f1(input):
    apply_gravity(input, lambda i, j: (i, j))
    return load(input)


def print_input(input):
    for line in input:
        print("".join(line))
    print()


def f2(input):
    seen = {}
    loads = {}
    state = 0

    while state < 1000000000:
        h = hash(input)
        if h in seen:
            seen_state = seen[h]
            cycle_length = state - seen_state
            remaining = 1000000000 - state
            remaining %= cycle_length
            return loads[seen_state + remaining]
        seen[h] = state
        loads[state] = load(input)
        state += 1
        cycle(input)

    return load(input)


def parse(s):
    return [[c for c in line] for line in s.splitlines()]


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
print(f2(input))
