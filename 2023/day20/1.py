import os

with open(os.path.dirname(__file__) + "/input.txt", "r") as input_file:
    p = {}

    for line in input_file:
        words = line.split(" ")
        typ = words[0][0]
        module = words[0][1:]
        targets = []
        for word in words[2:]:
            targets.append(word[:-1])
        p[module] = (typ, targets)

for module in p:
    typ, targets = p[module]
    state = None
    if typ == "%":
        state = "off"
    elif typ == "&":
        state = {}
        for j in p:
            if module in p[j][1]:
                state[j] = "low"
    p[module] = (typ, targets, state)


low = high = 0
from collections import deque as q

for _ in range(1000):  # button presses
    low += 1  # low pulse to broadcaster
    ops = q(())
    # (target,pulse,from)
    brodcast_targets = p["roadcaster"][1]
    for brodcast_target in brodcast_targets:
        ops.append((brodcast_target, "low", "roadcaster"))
    while ops:
        module, pulse, origin = ops.popleft()
        if pulse == "low":
            low += 1
        else:
            high += 1
        if module not in p:
            continue
        typ = p[module][0]
        targets = p[module][1]
        state = p[module][2]
        if typ == "%":
            if pulse == "low":
                if state == "off":
                    p[module] = (typ, targets, "on")
                    output = "high"
                else:
                    p[module] = (typ, targets, "off")
                    output = "low"

                for target in targets:
                    ops.append((target, output, module))

        elif typ == "&":
            p[module][2][origin] = pulse
            if all(x == "high" for x in p[module][2].values()):
                output = "low"
            else:
                output = "high"
            for target in targets:
                ops.append((target, output, module))

print(low * high)
