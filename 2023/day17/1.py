import heapq
import os

input = open(os.path.dirname(__file__) + "/input.txt", "r")
a = [list(map(int, line.strip())) for line in input]
n = len(a)

done = set()
direction = {"r": (1, 0), "l": (-1, 0), "u": (0, 1), "d": (0, -1), " ": (0, 0)}

p = [(0, 0, 0, 0, 0, 0)]

while True:
    heat_loss, ci, cj, fi, fj, streak = heapq.heappop(p)

    if (ci, cj) == (n - 1, n - 1):
        print(heat_loss)
        break

    if (ci, cj, fi, fj, streak) in done:
        continue

    done.add((ci, cj, fi, fj, streak))

    if streak < 3 and (fi, fj) != (0, 0):
        ni = ci + fi
        nj = cj + fj
        if 0 <= ni < n and 0 <= nj < n:
            heapq.heappush(p, (heat_loss + a[ni][nj], ni, nj, fi, fj, streak + 1))

    for _ in direction:
        if _ != " ":
            if direction[_] != (fi, fj) and direction[_] != (-fi, -fj):
                ni = direction[_][0] + ci
                nj = direction[_][1] + cj
                if 0 <= ni < n and 0 <= nj < n:
                    heapq.heappush(
                        p,
                        (
                            heat_loss + a[ni][nj],
                            ni,
                            nj,
                            direction[_][0],
                            direction[_][1],
                            1,
                        ),
                    )
