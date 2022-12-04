print(sum([((min0 <= min1 and max0 >= max1) or (min1 <= min0 and max1 >= max0)) for ((min0, max0), (min1, max1)) in [[[int(x) for x in w.split("-")] for w in l.split(",")]
           for l in open("input.txt").read().split("\n")]]))

print(sum([((min0 <= max1 and max0 >= min1) or (min1 <= max0 and max1 >= min0)) for ((min0, max0), (min1, max1)) in [[[int(x) for x in w.split("-")] for w in l.split(",")]
           for l in open("input.txt").read().split("\n")]]))
