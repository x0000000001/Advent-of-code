from math import prod
import sys

s = open(sys.argv[1]).read().strip()


def get_supports(input):
    input.sort(key=lambda line: line[0][2])
    supports = {}
    is_supported = {}
    inter = lambda l0, l1: prod(
        l0[1][k] >= l1[0][k] and l0[0][k] <= l1[1][k] for k in range(2)
    )

    for i in range(len(input)):
        intersects = [j for j in range(0, i) if inter(input[i], input[j])]
        highest = max(input[j][1][2] for j in intersects) if len(intersects) > 0 else 0
        diff = input[i][0][2] - (highest + 1)
        input[i][0][2] -= diff
        input[i][1][2] -= diff
        is_supported[i] = set()
        for support_j in [j for j in intersects if input[j][1][2] == highest]:
            if support_j not in supports:
                supports[support_j] = set()

            supports[support_j].add(i)
            is_supported[i].add(support_j)

    fallers = [
        i
        for i in range(len(input))
        if i in supports and not prod(len(is_supported[j]) > 1 for j in supports[i])
    ]

    return supports, is_supported, fallers


def f1(input):
    _, _, fallers = get_supports(input)
    return len(input) - len(fallers)


def get_chain(supports, is_supported, start):
    queue = [start]
    removed = set([start])

    while queue:
        i = queue.pop()

        if i not in supports:
            continue

        for child in supports[i]:
            if is_supported[child].issubset(removed):
                removed.add(child)
                queue.append(child)

    return len(removed) - 1


def f2(input):
    supports, is_supported, fallers = get_supports(input)
    return sum(get_chain(supports, is_supported, i) for i in fallers)


def parse(s):
    parse_pos = lambda w: [int(x) for x in w.split(",")]
    return [
        (parse_pos(a), parse_pos(b))
        for a, b in [line.split("~") for line in s.splitlines()]
    ]


input = parse(s)
print(f1(input))
print(f2(input))
