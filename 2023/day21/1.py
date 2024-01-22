import copy
import os

n = 131
a = [["0" for i in range(n)] for j in range(n)]
input_file = open(os.path.dirname(__file__) + "/input.txt", "r")
i = j = 0
si = sj = 0
for line in input_file:
    for char in line:
        if char != "\n":
            a[i][j % n] = char
            if char == "S":
                si = i
                sj = j
            j += 1
    i += 1
    j = 0

a[si][sj] = "O"


def step(garden=None, c=0):
    if garden is None:
        garden = a
    t = copy.deepcopy(garden)
    if c == 64:
        ans = 0
        for i in range(n):
            for j in range(n):
                if t[i][j] == "O":
                    ans += 1
        print(ans)
        return
    for i in range(n):
        for j in range(n):
            if garden[i][j] == "O":
                if i > 0:
                    if t[i - 1][j] != "#":
                        t[i - 1][j] = "O"
                if i < n - 1:
                    if t[i + 1][j] != "#":
                        t[i + 1][j] = "O"
                if j > 0:
                    if t[i][j - 1] != "#":
                        t[i][j - 1] = "O"
                if j < n - 1:
                    if t[i][j + 1] != "#":
                        t[i][j + 1] = "O"
                t[i][j] = "."
                garden[i][j] = "."

    step(t, c + 1)


step()
