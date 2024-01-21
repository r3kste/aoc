import os
from collections import (
    deque,
)  # just use queue based for loop instead of recursion... wow

n = 131
garden = [["0" for i in range(n)] for j in range(n)]
input = open(os.path.dirname(__file__) + "/input.txt", "r")
i = j = 0
si = sj = 0
for line in input:
    for char in line:
        if char != "\n":
            garden[i][j % n] = char
            if char == "S":
                si = i
                sj = j
            j += 1
    i += 1
    j = 0

garden[si][sj] = "."


def step(si, sj, ss):
    final = set()
    done = {(si, sj)}
    td = deque([(si, sj, ss)])

    while td:
        i, j, c = td.popleft()

        if c % 2 == 0:
            final.add((i, j))
        if c == 0:
            continue

        for ni, nj in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]:
            if (
                    ni < 0
                    or ni >= len(garden)
                    or nj < 0
                    or nj >= len(garden[0])
                    or garden[ni][nj] == "#"
                    or (ni, nj) in done
            ):
                continue
            done.add((ni, nj))
            td.append((ni, nj, c - 1))

    return len(final)


t = 26501365
l = t // n - 1

oc = (l // 2 * 2 + 1) ** 2
ec = ((l + 1) // 2 * 2) ** 2
ol = step(si, sj, n)
el = step(si, sj, n - 1)

cr = step(si, 0, n - 1)
cl = step(si, n - 1, n - 1)
ct = step(0, sj, n - 1)
cb = step(n - 1, sj, n - 1)

sr = step(0, 0, n // 2 - 1)
sl = step(0, n - 1, n // 2 - 1)
st = step(n - 1, 0, n // 2 - 1)
sb = step(n - 1, n - 1, n // 2 - 1)
ts = sr + sl + st + sb

lr = step(0, 0, n * 3 // 2 - 1)
ll = step(0, n - 1, n * 3 // 2 - 1)
lt = step(n - 1, 0, n * 3 // 2 - 1)
lb = step(n - 1, n - 1, n * 3 // 2 - 1)
tl = lr + ll + lt + lb

print(oc * ol + ec * el + cr + cl + ct + cb + (l + 1) * (ts) + l * tl)
