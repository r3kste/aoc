import os

input_file = open(os.path.dirname(__file__) + "/input.txt", "r")
data = {}
index = 1
for line in input_file:
    line = line[:-1].strip()
    coords, velocitis = line.split("@")
    coords = coords.strip()
    velocitis = velocitis.strip()
    coords = coords.split(",")
    velocitis = velocitis.split(",")
    coord = tuple([int(a.strip()) for a in coords])
    vel = tuple([int(a.strip()) for a in velocitis])
    data[index] = (
        coord,
        vel,
    )
    index += 1


def poi(line1, line2):
    line1 = data[line1]
    line2 = data[line2]
    x1, y1, _ = line1[0]
    x2, y2, _ = line2[0]
    a1, b1, _ = line1[1]
    a2, b2, _ = line2[1]
    q2 = float(b2) / float(a2)
    q1 = float(b1) / float(a1)
    if q1 != q2:
        x = (q2 * x2 - q1 * x1 + y1 - y2) / (q2 - q1)
        y = q2 * (x - x2) + y2
        sa1 = a1 / abs(a1)
        sa2 = a2 / abs(a2)
        sb1 = b1 / abs(b1)
        sb2 = b2 / abs(b2)
        if (
                sa1 * x > sa1 * x1
                and sa2 * x > sa2 * x2
                and sb1 * y > sb1 * y1
                and sb2 * y > sb2 * y2
        ):
            return x, y
        else:
            return 0, 0
    else:
        return 0, 0


c = 0
s, e = (200000000000000, 400000000000000)
# s, e = (7, 27)

for i in data:
    for j in data:
        if i < j:
            x, y = poi(i, j)
            if s <= x <= e and s <= y <= e:
                c += 1
print(c)
