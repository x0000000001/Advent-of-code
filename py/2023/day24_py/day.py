import sys
from pprint import pprint

s = open(sys.argv[1]).read().strip()


def f1(input):
    points = [(line[0], [c + d for c, d in zip(line[0], line[1])]) for line in input]
    count, n = 0, len(input)
    in_bound = lambda x: x >= 200000000000000 and x <= 400000000000000  # noqa: E731
    for i in range(n):
        for j in range(i + 1, n):
            px, py = intersects_plan(points[i], points[j], True, 0, 1)

            if px is None:
                continue

            if in_bound(px) and in_bound(py):
                count += 1

    return count


is_pos = lambda x1, x2, px: (x2 - x1) * (px - x1) >= 0  # noqa: E731


def intersects_plan(line1, line2, pos, index0, index1):
    ((x1, y1, _), (x2, y2, _)) = line1
    x1 = line1[0][index0]
    y1 = line1[0][index1]
    x2 = line1[1][index0]
    y2 = line1[1][index1]
    x3 = line2[0][index0]
    y3 = line2[0][index1]
    x4 = line2[1][index0]
    y4 = line2[1][index1]
    den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)

    if den == 0:
        if (x1 - x3) * (y1 - y2) == (x1 - x2) * (y1 - y3):
            # same line
            return "a", "a"

        return None, None

    px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / den

    py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / den

    if pos and (not is_pos(x3, x4, px) or not is_pos(x1, x2, px)):
        return None, None

    return px, py


def get_t(x1, x2, v1, v2):
    if v1 == v2:
        if x1 == x2:
            return -1
        else:
            return None

    elif (x1 - x2) % (v2 - v1) != 0:
        return None

    return (x1 - x2) // (v2 - v1)


def inter_point(line1, line2, d_speed):
    ts = [
        get_t(
            line1[0][i], line2[0][i], line1[1][i] - d_speed[i], line2[1][i] - d_speed[i]
        )
        for i in range(3)
    ]

    ts = [t for t in ts if t != -1]

    if len(ts) == 0:
        return ["a", "a", "a"]

    t_ref = ts[0]

    for t in ts:
        if t is None or t != t_ref:
            return [None, None, None]

    return [
        x + v * t_ref
        for x, v in [(line1[0][i], line1[1][i] - d_speed[i]) for i in range(3)]
    ]


def get_2_points(line, dspeed):
    return (line[0], [line[0][i] + line[1][i] - dspeed[i] for i in range(3)])


def f2(input):
    d, u = -500, 500
    n = len(input)
    possible_dspeeds = []

    for vx in range(d, u):
        for vy in range(d, u):
            dspeed = [vx, vy, 0]
            ref_inter = intersects_plan(
                get_2_points(input[0], dspeed),
                get_2_points(input[1], dspeed),
                False,
                0,
                1,
            )
            is_good = True

            for i in range(n):
                line1 = get_2_points(input[i], dspeed)
                for j in range(i + 1, n):
                    line2 = get_2_points(input[j], dspeed)
                    inter = intersects_plan(line1, line2, False, 0, 1)

                    if ref_inter[0] == "a":
                        ref_inter = inter

                    if inter[0] != "a" and ref_inter != inter:
                        is_good = False
                        break
                if not is_good:
                    break
            else:
                possible_dspeeds.append((ref_inter[0], dspeed))

    for x, dspeed in possible_dspeeds:
        for vz in range(d, u):
            dspeed[2] = vz

            ref_inter = intersects_plan(
                get_2_points(input[0], dspeed),
                get_2_points(input[1], dspeed),
                False,
                1,
                2,
            )
            is_good = True

            for i in range(n):
                line1 = get_2_points(input[i], dspeed)
                for j in range(i + 1, n):
                    line2 = get_2_points(input[j], dspeed)
                    inter = intersects_plan(line1, line2, False, 1, 2)

                    if ref_inter[0] == "a":
                        ref_inter = inter

                        is_good = False
                    if inter[0] != "a" and ref_inter != inter:
                        break
                if not is_good:
                    break
            else:
                return int(sum(ref_inter) + x)


def parse(s):
    coords = lambda w: [int(x) for x in w.split(", ")]  # noqa: E731
    return [[coords(w) for w in line.split("@")] for line in s.splitlines()]


input = parse(s)
print(f1(input))
print(f2(input))
