import sys
from pprint import pprint


def propagate(input, y, x, dir, energized, seen):
    if (x, y, dir) in seen:
        return

    n = len(input)

    while x >= 0 and y >= 0 and x < n and y < n:
        energized.add((x, y))
        seen.add((x, y, dir))

        c = input[y][x]

        if (dir == 1 or dir == 3) and c == "|":
            propagate(input, y - 1, x, 0, energized, seen)
            propagate(input, y + 1, x, 2, energized, seen)
            break

        elif (dir == 0 or dir == 2) and c == "-":
            propagate(input, y, x - 1, 3, energized, seen)
            propagate(input, y, x + 1, 1, energized, seen)
            break

        elif c == "/":
            if dir == 1:
                y -= 1
                dir = 0
            elif dir == 3:
                y += 1
                dir = 2
            elif dir == 0:
                x += 1
                dir = 1
            elif dir == 2:
                x -= 1
                dir = 3
        elif c == "\\":
            if dir == 3:
                y -= 1
                dir = 0
            elif dir == 1:
                y += 1
                dir = 2
            elif dir == 2:
                x += 1
                dir = 1
            elif dir == 0:
                x -= 1
                dir = 3
        else:
            if dir == 0:
                y -= 1
            elif dir == 1:
                x += 1
            elif dir == 2:
                y += 1
            elif dir == 3:
                x -= 1


def energy(input, x, y, dir):
    energized = set()
    seen = set()
    propagate(input, y, x, dir, energized, seen)

    return len(energized)


def f1(input):
    return energy(input, 0, 0, 1)


def f2(input):
    max_energy = 0
    n = len(input)

    for i in range(n):
        max_energy = max(max_energy, energy(input, 0, i, 1))
        max_energy = max(max_energy, energy(input, n - 1, i, 3))
        max_energy = max(max_energy, energy(input, i, 0, 2))
        max_energy = max(max_energy, energy(input, i, n - 1, 0))

    return max_energy


def parse(s):
    return [[c for c in line] for line in s.splitlines()]


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
pprint(input)
print(f1(input))
print(f2(input))
