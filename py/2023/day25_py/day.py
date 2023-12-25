import sys
from collections import deque
import graphviz as gv
from pprint import pprint

s = open(sys.argv[1]).read().strip()


def component(edges, start, removed_edges):
    queue = [start]
    component = set([start])

    while queue:
        x = queue.pop()

        for y in edges[x]:
            if (y, x) in removed_edges or (x, y) in removed_edges:
                continue

            if y not in component:
                component.add(y)
                queue.append(y)

    return component


def count_shortest_paths(edges, start, counts):
    queue = deque([(0, start)])
    scores = {}
    prev = {start: ""}

    while queue:
        score, x = queue.popleft()

        new_score = score + 1
        edge = tuple(sorted((x, prev[x])))
        if edge not in counts:
            counts[edge] = 0

        counts[edge] += score
        for y in edges[x]:
            if y in scores and scores[y] <= new_score:
                continue

            scores[y] = new_score
            prev[y] = x
            queue.append((new_score, y))


def are_good(edges, removed_edges):
    start0 = next(iter(edges))
    component0 = component(edges, start0, removed_edges)

    if len(component0) == len(edges):
        return None

    start1 = start0

    for x in edges:
        if x not in component0:
            start1 = x
            break

    component1 = component(edges, start1, removed_edges)

    if len(component0) + len(component1) == len(edges):
        return len(component0) * len(component1)

    return None


def f1(input):
    vertices = set()
    edges = {}

    for x, ys in input:
        vertices.add(x)

        if x not in edges:
            edges[x] = set()

        for y in ys:
            vertices.add(y)

            if y not in edges:
                edges[y] = set()

            edges[x].add(y)
            edges[y].add(x)

    counts = {}

    for x in vertices:
        count_shortest_paths(edges, x, counts)

    edges_sorted = [p for p in list(counts.items()) if p[0][0] != ""]
    edges_sorted.sort(key=lambda pair: pair[1], reverse=True)

    n = len(edges_sorted)
    for i in range(2, n):
        for j in range(1, i):
            for k in range(j):
                res = are_good(edges, {edges_sorted[w][0] for w in [i, j, k]})

                if res:
                    return res


def parse(s):
    lines = []

    for line in s.splitlines():
        w = line.split(" ")
        lines.append((w[0][:-1], w[1:]))

    return lines


input = parse(s)
print(f1(input))
