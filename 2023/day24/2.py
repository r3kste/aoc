import os

import sympy

hailstones = [
    tuple(map(int, line.replace("@", ",").split(",")))
    for line in open(os.path.dirname(__file__) + "/input.txt", "r")
]

xr, yr, zr, vxr, vyr, vzr = sympy.symbols("xr, yr, zr, vxr, vyr, vzr")

equations = []

for i, (sx, sy, sz, vx, vy, vz) in enumerate(hailstones):
    equations.append((xr - sx) * (vy - vyr) - (yr - sy) * (vx - vxr))
    equations.append((yr - sy) * (vz - vzr) - (zr - sz) * (vy - vyr))
    if i < 2:
        continue
    answers = [
        soln
        for soln in sympy.solve(equations)
        if all(x % 1 == 0 for x in soln.values())
    ]
    if len(answers) == 1:
        print(answers[0][xr] + answers[0][yr] + answers[0][zr])
        break
