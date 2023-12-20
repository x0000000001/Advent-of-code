import sys
from collections import deque
from math import prod
from pprint import pprint

# import graphviz as gv


def init_state(modules):
    states = {}

    for name, (mod_type, targets) in modules.items():
        if mod_type == "&":
            states[name] = {}

    for name, (mod_type, targets) in modules.items():
        if mod_type == "%":
            states[name] = False

        for t in targets:
            if t not in modules:
                continue

            if modules[t][0] == "&":
                states[t][name] = False

    return states


def push_button(modules, states, hb_not_zero):
    messages = deque()
    messages.append(("button", "broadcaster", False))

    count_low = 0
    count_high = 0

    while messages:
        sender, receiver, is_high = messages.popleft()

        if hb_not_zero is not None and receiver == "hb" and is_high:
            hb_not_zero.add(sender)

        if is_high:
            count_high += 1
        else:
            count_low += 1

        if receiver not in modules:
            continue

        rcv_type, rcv_targets = modules[receiver]

        if rcv_type == "%":
            if is_high:
                continue

            states[receiver] = not states[receiver]
            snd_type = states[receiver]

            for t in rcv_targets:
                messages.append((receiver, t, snd_type))

        elif rcv_type == "&":
            states[receiver][sender] = is_high

            snd_type = not bool(prod(states[receiver].values()))

            for t in rcv_targets:
                messages.append((receiver, t, snd_type))

        else:
            for t in rcv_targets:
                messages.append((receiver, t, is_high))

    return count_low, count_high, hb_not_zero


def f1(modules):
    states = init_state(modules)
    count_low, count_high = 0, 0

    for _ in range(1000):
        tmp_low, tmp_high, _ = push_button(modules, states, None)
        count_low += tmp_low
        count_high += tmp_high

    return count_low * count_high


def f2(modules):
    states = init_state(modules)

    # g = gv.Digraph("G")
    #
    # for name, (t, _) in modules.items():
    #     if t == "&":
    #         shape = "square"
    #     elif t == "%":
    #         shape = "circle"
    #     else:
    #         shape = "doublecircle"
    #
    #     g.attr("node", shape=shape)
    #     g.node(name)
    #
    # for name, (_, targets) in modules.items():
    #     for t in targets:
    #         g.edge(name, t)
    #
    # g.view()

    i = 0
    cycles = {}

    while True:
        i += 1
        _, _, not_zero = push_button(modules, states, set())

        for key in not_zero:
            if key not in cycles:
                cycles[key] = i

        if len(cycles) == 4:
            return prod(cycles.values())


def parse(s):
    modules = {}

    for line in s.splitlines():
        w = line.split(" -> ")
        c = w[0][0]

        if not c.isalpha():
            mod_type = c
            mod_name = w[0][1:]
        else:
            mod_type = "bc"
            mod_name = w[0]

        targets = list(w[1].split(", "))

        modules[mod_name] = (mod_type, targets)

    return modules


with open(sys.argv[1]) as f:
    s = f.read().strip()

input = parse(s)
print(f1(input))
print(f2(input))
