import time
import copy


def isvalid(destination, obj1, obj2, floors, elevator):
    next_floor = list(floors[destination])
    next_floor.append(obj1)
    if obj2 != "":
        next_floor.append(obj2)

    for i in next_floor:
        if i[1] == "M":
            for j in next_floor:
                if j[1] == "G" and i[0] + "G" not in next_floor:
                    return False

    if obj1 not in floors[elevator]:
        return False
    if obj2 != "" and obj2 not in floors[elevator]:
        return False

    cur_floor = list(floors[elevator])
    cur_floor.remove(obj1)
    if obj2 != "":
        cur_floor.remove(obj2)
    for i in cur_floor:
        if i[1] == "M" and i[0] + "G" not in cur_floor:
            for j in cur_floor:
                if j != i and j[1] == "G":
                    return False
    return True


def move(destination, obj1, obj2, floors, elevator, steps):
    next_floors = copy.deepcopy(floors)
    next_elevator = int(elevator)
    state = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], destination]

    if not isvalid(destination, obj1, obj2, next_floors, next_elevator):
        return ((), (), ())
    else:
        next_floors[destination].append(obj1)
        if obj2 != "":
            next_floors[destination].append(obj2)
        next_floors[next_elevator].remove(obj1)
        if obj2 != "":
            next_floors[next_elevator].remove(obj2)
        next_floors[destination] = sorted(next_floors[destination])
        next_elevator = destination

    for i in range(4):
        for j in next_floors[i]:
            if j[1] == "M":
                if j[0] + "G" not in next_floors[i]:
                    state[i][0] += 1
                else:
                    state[i][2] += 1
            else:
                if j[0] + "M" not in next_floors[i]:
                    state[i][1] += 1

    return (next_floors, next_elevator, state, steps)


def find_path(floors):
    steps = 1
    start_elevator = 0
    moves = []
    seen = [()]
    found = False
    amount = sum(len(x) for x in floors)

    for i in floors[start_elevator]:
        next = move(1, i, "", floors, start_elevator, steps)
        if next[2] not in seen:
            moves.append(next)
            seen.append(next[2])
        for j in floors[start_elevator]:
            if i != j:
                next = move(1, i, j, floors, start_elevator, steps)
                if next[2] not in seen:
                    moves.append(next)
                    seen.append(next[2])

    while not found:
        steps += 1
        for i in copy.deepcopy(moves):
            moves.remove(i)
            if len(i[0][3]) == amount:
                return i[3]
            floors = i[0]
            next_floors = copy.deepcopy(floors)
            elevator = int(i[1])

            for obj1 in next_floors[elevator]:
                for obj2 in next_floors[elevator]:
                    if obj1 != obj2:
                        if elevator < 3:
                            next = move(
                                elevator + 1,
                                obj1,
                                obj2,
                                next_floors,
                                elevator,
                                steps,
                            )
                            if next[2] not in seen:
                                moves.append(next)
                                seen.append(next[2])

                if elevator < 3:
                    next = move(elevator + 1, obj1, "", next_floors, elevator, steps)
                    if next[2] not in seen:
                        moves.append(next)
                        seen.append(next[2])

                next_floors = copy.deepcopy(floors)
                if (
                    all(next_floors[x] == [] for x in range(1, elevator))
                    and elevator > 1
                ):
                    continue

                if elevator > 0:
                    next = move(elevator - 1, obj1, "", next_floors, elevator, steps)
                    if next[2] not in seen:
                        moves.append(next)
                        seen.append(next[2])

                for obj2 in next_floors[elevator]:
                    if obj1[0] != obj2[0]:
                        if elevator > 0:
                            next = move(
                                elevator - 1,
                                obj1,
                                obj2,
                                next_floors,
                                elevator,
                                steps,
                            )
                            if next[2] not in seen:
                                moves.append(next)
                                seen.append(next[2])


print("Day 11:")

start_time = time.time()

floors = [["SG", "SM", "PG", "PM"], ["TG", "RG", "RM", "CG", "CM"], ["TM"], []]
print(f"Part 1: {find_path(floors)}")


floors[0].extend(("EG", "EM", "DG", "DM"))
print(f"Part 2: {find_path(floors)}")
print("Run time: %s" % (time.time() - start_time))
