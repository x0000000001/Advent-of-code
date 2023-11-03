print(max([sum([int(s) for s in x.split("\n")])
           for x in open("input.txt").read().split("\n\n")]))
print(sum(sorted([sum([int(s) for s in x.split("\n")])
                  for x in open("input.txt").read().split("\n\n")])[-1:-4:-1]))
