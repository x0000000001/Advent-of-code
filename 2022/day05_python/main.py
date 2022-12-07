from functools import reduce


print(open("test_input0.txt").read().splitlines())
print(reduce(lambda s, l: s[l[2]-1] += s[l[1]-1], [(int(l[1]), int(l[3]), int(l[5])) for l in [l.split(" ")
      for l in open("test_input1.txt").read().splitlines()]], open("test_input0.txt").read().splitlines()))
