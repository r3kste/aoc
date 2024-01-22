import os

import numpy as np

input_file = open(os.path.dirname(__file__) + "/input.txt", "r")
cubes = {}
supportedby = {}
supportsfor = {}
brick = 1
for line in input_file:
    left, right = line[:-1].split("~")
    start = list(map(int, (left.split(","))))
    end = list(map(int, (right.split(","))))
    cubes[brick] = (
        tuple(start),
        tuple(end),
    )
    supportedby[brick] = set()
    supportsfor[brick] = set()
    brick += 1

size = (10, 10, 300)
space = np.zeros(size, dtype=int)

for brick in cubes:
    start, end = cubes[brick]
    sx, sy, sz = start
    ex, ey, ez = end
    for x in range(sx, ex + 1):
        for y in range(sy, ey + 1):
            for z in range(sz, ez + 1):
                space[x][y][z] = brick


def settle(cube):
    start, end = cubes[cube]
    sx, sy, sz = start
    ex, ey, ez = end
    for x in range(sx, ex + 1):
        for y in range(sy, ey + 1):
            for z in range(sz, ez + 1):
                if sz <= 1 or (space[x][y][z - 1] != 0 and space[x][y][z - 1] != cube):
                    return 0
    for x in range(sx, ex + 1):
        for y in range(sy, ey + 1):
            for z in range(sz, ez + 1):
                space[x][y][z - 1] = space[x][y][z]
                cubes[cube] = ((sx, sy, sz - 1), (ex, ey, ez - 1))
                space[x][y][z] = 0
    return 1


def settleall():
    t = 0
    for z in range(1, size[2]):
        for x in range(size[0]):
            for y in range(size[1]):
                cube = space[x][y][z]
                if cube != 0:
                    c = settle(cube)
                    t += c
    if t == 0:
        return
    else:
        settleall()


def xview(x):
    for z in reversed(range(size[2])):
        for y in range(size[1]):
            if space[x][y][z] != 0:
                print(space[x][y][z], end="")
            else:
                print(".", end="")
            print(" ", end="")
        print(" ")


settleall()

for brick in cubes:
    start, end = cubes[brick]
    sx, sy, sz = start
    ex, ey, ez = end
    for x in range(sx, ex + 1):
        for y in range(sy, ey + 1):
            for z in range(sz, ez + 1):
                if space[x][y][z - 1] != 0 and space[x][y][z - 1] != brick:
                    supportedby[brick].add(space[x][y][z - 1])
                    supportsfor[space[x][y][z - 1]].add(brick)

ans = 0
for brick in supportsfor:
    for support in supportsfor[brick]:
        if len(supportedby[support]) == 1:
            break
    else:
        ans += 1

print(ans)
