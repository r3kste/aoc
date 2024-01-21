import os

file = os.path.dirname(__file__) + "/input.txt"
input_file = open(file, "r")
t = open(file, "r")
n = 0
for line in t:
    n = len(line) - 1
    t.close()
    break

grid = [["" for _ in range(n)] for __ in range(n)]
i = j = 0
for line in input_file:
    t = line.split("\n")[0]
    for char in t:
        grid[i][j] = char
        j += 1
    i += 1
    j = 0
input_file.close()

start = (0, 1)
end = (n - 1, n - 2)
poi = [start, end]
for i in range(n):
    for j in range(n):
        c = 0
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            ni = i + di
            nj = j + dj
            if ni in range(0, n) and nj in range(0, n):
                if grid[ni][nj] != "#":
                    c += 1
        if c >= 3:
            poi.append((i, j))

distance = {point: {} for point in poi}

dirs = {">": [(0, 1)], "v": [(1, 0)], ".": [(0, 1), (0, -1), (1, 0), (-1, 0)], "#": []}

for si, sj in poi:
    todo = [(int(0), si, sj)]
    seen = {(si, sj)}
    while todo:
        dist, i, j = todo.pop()

        if dist != 0 and (i, j) in poi:
            distance[(si, sj)][(i, j)] = dist
            continue

        for di, dj in dirs[grid[i][j]]:
            ni = i + di
            nj = j + dj
            if (
                ni in range(0, n)
                and nj in range(0, n)
                and (ni, nj) not in seen
                and grid[ni][nj] != "#"
            ):
                todo.append((dist + 1, ni, nj))
                seen.add((ni, nj))


def f(point):
    if point == end:
        return 0

    maxx = -float("inf")

    for next_point in distance[point]:
        maxx = max(maxx, f(next_point) + distance[point][next_point])

    return maxx


print(f(start))
