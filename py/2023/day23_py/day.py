from collections import deque
from math import prod
import sys
from pprint import pprint

s = open(sys.argv[1]).read().strip()


def reduce_graph(point, new_vertices, new_edges, neighbors_f, start, end):
    new_vertices.add(point)
    new_edges[point] = {}

    for score, p in neighbors_f(point):
        previous = point
        while True:
            nexts = [(w, np) for (w, np) in neighbors_f(p) if np != previous]

            if len(nexts) > 1 or p == end or p == start:
                new_edges[point][p] = score
                if p not in new_vertices:
                    reduce_graph(p, new_vertices, new_edges, neighbors_f, start, end)
                break
            elif not nexts:
                break

            previous = p
            w, p = nexts.pop()
            score += w


acceptable = set([(1, "v"), (-1, "^"), (1j, ">"), (-1j, "<")])


def neighbors(p, input, slopes):
    for d in (1, -1, 1j, -1j):
        neighbor = p + d
        w = 1

        if neighbor not in input:
            continue

        c = input[neighbor]

        if c == "#":
            continue
        elif slopes and c != ".":
            if (d, c) not in acceptable:
                continue
            w = 2
            neighbor += d

        yield w, neighbor


def longest_path(input, neighbors_f):
    start = [p for p, c in input.items() if p.real == 0 and c == "."][0]
    end = [
        p for p, c in input.items() if p.real == max(x.real for x in input) and c == "."
    ][0]

    new_edges, new_vertices = {}, set()
    reduce_graph(start, new_vertices, new_edges, neighbors_f, start, end)
    queue = deque([(0, start, {start})])
    max_score = 0

    while queue:
        score, pos, previous = queue.popleft()

        if pos == end:
            max_score = max(max_score, score)

        for neighbor, distance in new_edges[pos].items():
            if neighbor in previous:
                continue

            new_set = previous.union({neighbor})
            queue.append((score + distance, neighbor, new_set))

    return max_score


def f1(input):
    return longest_path(input, lambda p: neighbors(p, input, True))


def f2(input):
    return longest_path(input, lambda p: neighbors(p, input, False))


def parse(s):
    return {
        i + j * 1j: c
        for i, line in enumerate(s.splitlines())
        for j, c in enumerate(line)
    }


input = parse(s)
print(f1(input))
print(f2(input))
