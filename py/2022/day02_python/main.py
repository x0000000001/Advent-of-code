print(sum([(y-87) + {0: 3, 2: 6, 1: 0}[((x-65) - (y-88) + 3) % 3] for (x, y) in [map(ord, l.split(" "))
                                                                                 for l in open("input.txt").read().split("\n")]]))
print(sum([y+1 + {0: 3, 2: 6, 1: 0}[(x - y + 3) % 3]for (x, y) in [(x-65, (x - 65 + 2 + y-88) % 3) for (x, y) in [map(ord, l.split(" "))
                                                                                                                  for l in open("input.txt").read().split("\n")]]]))
