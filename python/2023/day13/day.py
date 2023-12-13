import sys

with open(sys.argv[1]) as f1:
    s = f1.read().strip()


def row(pattern):
    n = len(pattern)
    for i in range(n - 1):
        sym_len = min(i + 1, n - (i + 1))
        for j in range(sym_len):
            if pattern[i - j] != pattern[i + 1 + j]:
                break
        else:
            return i + 1


def row1(pattern):
    n = len(pattern)
    for i in range(n - 1):
        sym_len = min(i + 1, n - (i + 1))
        count = 0
        for j in range(sym_len):
            count += sum(
                [
                    pattern[i - j][k] != pattern[i + 1 + j][k]
                    for k in range(len(pattern[0]))
                ]
            )
        if count == 1:
            return i + 1


def f1(pats):
    r = 0
    c = 0
    for p in pats:
        pats_col = list(zip(*p))
        x = row(p)
        if x is not None:
            r += x
        else:
            c += row(pats_col)

    print(100 * r + c)


def f2(pats):
    r = 0
    c = 0
    for p in pats:
        pats_col = list(zip(*p))
        x = row1(p)
        if x is not None:
            r += x
        else:
            c += row1(pats_col)

    print(100 * r + c)


def parse(s):
    return [
        [[c for c in line] for line in block.splitlines()] for block in s.split("\n\n")
    ]


input = parse(s)
f1(input)
f2(input)
