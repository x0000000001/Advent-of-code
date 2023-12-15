import sys
from pprint import pprint


def hash(s):
    h = 0
    for c in s:
        h += ord(c)
        h *= 17
        h %= 256
    return h


def f1(input):
    return sum([hash(s) for s in input])


def f2(input):
    boxes = {}

    for instr in input:
        if instr[-1] == "-":
            key = instr[:-1]
            h = hash(key)
            if h not in boxes:
                continue
            for i, el in enumerate(boxes[h]):
                if el[0] == key:
                    boxes[h].pop(i)
                    break
        else:
            words = instr.split("=")
            key = words[0]
            n = int(words[1])
            h = hash(key)

            if h not in boxes:
                boxes[h] = [(key, n)]
                continue

            for i, el in enumerate(boxes[h]):
                if el[0] == key:
                    boxes[h][i] = (key, n)
                    break
            else:
                boxes[h].append((key, n))

    return sum(
        [
            (h + 1) * sum([lens[1] * (i + 1) for (i, lens) in enumerate(lenses)])
            for (h, lenses) in boxes.items()
        ]
    )


def parse(s):
    return s.splitlines()[0].split(",")


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
print(f2(input))
