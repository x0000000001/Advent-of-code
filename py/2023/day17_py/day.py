import sys
from heapq import heappush, heappop


def neighbors0(node, n):
    x, y, dir, streak = node
    if y > 0 and dir != 2:
        if dir == 0:
            if streak < 3:
                yield (x, y - 1, 0, streak + 1)
        else:
            yield (x, y - 1, 0, 1)
    if x > 0 and dir != 1:
        if dir == 3:
            if streak < 3:
                yield (x - 1, y, 3, streak + 1)
        else:
            yield (x - 1, y, 3, 1)
    if y < n - 1 and dir != 0:
        if dir == 2:
            if streak < 3:
                yield (x, y + 1, 2, streak + 1)
        else:
            yield (x, y + 1, 2, 1)
    if x < n - 1 and dir != 3:
        if dir == 1:
            if streak < 3:
                yield (x + 1, y, 1, streak + 1)
        else:
            yield (x + 1, y, 1, 1)


def neighbors1(node, n):
    x, y, dir, streak = node
    if streak < 4:
        if dir == 0 and y > 0:
            yield (x, y - 1, 0, streak + 1)
        elif dir == 1 and x < n - 1:
            yield (x + 1, y, 1, streak + 1)
        elif dir == 2 and y < n - 1:
            yield (x, y + 1, 2, streak + 1)
        elif dir == 3 and x > 0:
            yield (x - 1, y, 3, streak + 1)

    else:
        if y > 0 and dir != 2:
            if dir == 0:
                if streak < 10:
                    yield (x, y - 1, 0, streak + 1)
            else:
                yield (x, y - 1, 0, 1)
        if x > 0 and dir != 1:
            if dir == 3:
                if streak < 10:
                    yield (x - 1, y, 3, streak + 1)
            else:
                yield (x - 1, y, 3, 1)
        if y < n - 1 and dir != 0:
            if dir == 2:
                if streak < 10:
                    yield (x, y + 1, 2, streak + 1)
            else:
                yield (x, y + 1, 2, 1)
        if x < n - 1 and dir != 3:
            if dir == 1:
                if streak < 10:
                    yield (x + 1, y, 1, streak + 1)
            else:
                yield (x + 1, y, 1, 1)


def a_star(map, start_nodes, end, neighbors_f):
    queue = []
    n = len(map)
    scores = {}
    for ss in start_nodes:
        heappush(queue, (0, ss))
        scores[ss] = 0

    while queue:
        score, node = heappop(queue)
        if (node[0], node[1]) == end:
            return score
        for new_node in neighbors_f(node, n):
            new_score = score + map[new_node[1]][new_node[0]]
            if new_node in scores and scores[new_node] <= new_score:
                continue
            scores[new_node] = new_score
            heappush(queue, (new_score, new_node))

    return None


start_nodes = [(0, 0, dir, 0) for dir in range(4)]


def f1(input):
    n = len(input)
    return a_star(
        input,
        start_nodes,
        (n - 1, n - 1),
        neighbors0,
    )


def f2(input):
    n = len(input)
    return a_star(input, start_nodes, (n - 1, n - 1), neighbors1)


def parse(s):
    return [[int(c) for c in line] for line in s.splitlines()]


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
print(f2(input))
