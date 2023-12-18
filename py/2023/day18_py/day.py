import sys
from pprint import pprint


def f1(input):
    instrs = [(dir, length) for dir, length, _ in input]
    return area(instrs)


def area(instrs):
    path, path_len = get_path(instrs)
    area = shoelace(path)
    inside = area + 1 - (path_len // 2)
    return inside + path_len


def shoelace(path):
    area = 0

    for i in range(len(path)):
        area += (path[i][1] + path[(i + 1) % len(path)][1]) * (
            path[i][0] - path[(i + 1) % len(path)][0]
        )

    return abs(area) // 2


def get_path(instrs):
    path = []
    x = 0
    y = 0
    path.append((0, 0))
    path_len = 0

    for dir, length in instrs:
        pprint(f"{dir} {length}")

        if dir == 0:
            x += length
        elif dir == 1:
            y += length
        elif dir == 2:
            x -= length
        elif dir == 3:
            y -= length

        path.append((x, y))
        path_len += length

    path.pop()

    return path, path_len


def f2(input):
    instrs = [(int(hex[-1]), int(hex[:-1], 16)) for _, _, hex in input]
    return area(instrs)


def parse(s):
    input = []

    char_to_dir = {"R": 0, "D": 1, "L": 2, "U": 3}

    for line in s.splitlines():
        w = line.split()
        input.append((char_to_dir[w[0]], int(w[1]), w[2][2:-1]))

    return input


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
print(f2(input))
