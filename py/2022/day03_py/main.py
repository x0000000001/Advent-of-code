print(sum([list(set(r[:len(r)//2]) & set(r[len(r)//2:]))[0] for r in [[ord(x) - 64 if ord(x) < 97 else ord(x) - 70 for x in l]
                                                                      for l in open("input.txt").read().swapcase().split("\n")]]))
print(sum([list(set(r0) & set(r1) & set(r2))[0] for (r0, r1, r2) in zip(*(iter([[ord(x) - 64 if ord(x) < 97 else ord(x) - 70 for x in l]
                                                                                for l in open("input.txt").read().swapcase().split("\n")]),) * 3)]))
