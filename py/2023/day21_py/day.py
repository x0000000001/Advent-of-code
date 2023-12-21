import sys
from collections import deque


def neighbors(map, i, j):
    n = len(map)
    is_valid = lambda x: x >= 0 and x < n

    for d in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
        new_i, new_j = [sum(x) for x in zip(d, (i, j))]
        if is_valid(new_i) and is_valid(new_j) and map[new_i][new_j] != "#":
            yield new_i, new_j


def weird_djikstra(map, start, steps, neighbors_f):
    count = 0
    queue = deque()
    queue.append((start, 0))
    scores = {}
    seen = set()

    while queue:
        (i, j), score = queue.popleft()

        if (i, j) in seen:
            continue

        if score > steps:
            break

        seen.add((i, j))

        if (score + steps) % 2 == 0:
            count += 1

        for ni, nj in neighbors_f(map, i, j):
            if (ni, nj) in seen:
                continue

            if (ni, nj) in scores and scores[(ni, nj)] <= score + 1:
                continue

            scores[(ni, nj)] = score + 1
            queue.append(((ni, nj), score + 1))

    return count


def f1(input):
    map, start = input
    return weird_djikstra(map, start, 64, neighbors)


def neighbors1(map, i, j):
    n = len(map)
    oi = i % n
    oj = j % n

    new_i = (oi - 1) % n
    if map[new_i][oj] == ".":
        yield (i - 1, j)
    new_i = (oi + 1) % n
    if map[new_i][oj] == ".":
        yield (i + 1, j)
    new_j = (oj - 1) % n
    if map[oi][new_j] == ".":
        yield (i, j - 1)
    new_j = (oj + 1) % n
    if map[oi][new_j] == ".":
        yield (i, j + 1)


# quadratic interpolation code : https://www2.lawrence.edu/fast/GREGGJ/CMSC210/arithmetic/interpolation.html
def f2(parsed_input):
    map, start = parsed_input
    steps = 26501365
    n = len(map)
    y0, y1, y2 = [
        weird_djikstra(map, start, steps % n + i * n, neighbors1) for i in range(3)
    ]
    diff10 = y1 - y0
    diff21 = y2 - y1
    c = y0
    b = diff10
    a = (diff21 - diff10) // 2
    x = steps // n
    return a * (x - 1) * x + b * x + c


def parse(s):
    map = [[c for c in line] for line in s.splitlines()]
    n = len(map)
    start = None

    for i in range(n):
        for j in range(n):
            if map[i][j] == "S":
                start = (i, j)
                map[i][j] = "."
                break
    return map, start


with open(sys.argv[1]) as f:
    s = f.read().strip()

parsed_input = parse(s)
print(f1(parsed_input))
print(f2(parsed_input))
