import os


input_file = open(os.path.dirname(__file__) + "/input.txt", "r")
output_file = open(os.path.dirname(__file__) + "/output.txt", "w")

circuit = {}
all_nodes = set()
for line in input_file:
    components = line[:-1].split(" ")
    components[0] = components[0][:-1]
    circuit[components[0]] = components[1:]
    for _ in components:
        all_nodes.add(_)

for component in circuit:
    for connection in circuit[component]:
        print(component, "->", connection, file=output_file)

#####
# zxb -> zkv
# lkf -> scf
# pgl -> mtl
#####

split_circuit = {}
for component in circuit:
    split_circuit[component] = []
    for node in circuit[component]:
        split_circuit[node] = []

simplified = open(os.path.dirname(__file__) + "/split.txt", "r")
for line in simplified:
    one = line[:3]
    two = line[-4:-1]
    split_circuit[one].append(two)
    split_circuit[two].append(one)

cluster1 = set()
start = "scf"
cluster1.add(start)
todo = [start]

while todo:
    node = todo.pop()
    cluster1.add(node)
    if node in split_circuit:
        for connection in split_circuit[node]:
            if connection not in cluster1:
                todo.append(connection)

a = len(cluster1)

cluster2 = set()
start = "zxb"
cluster2.add(start)
todo = [start]
while todo:
    node = todo.pop()
    cluster2.add(node)
    if node in split_circuit:
        for connection in split_circuit[node]:
            if connection not in cluster2:
                todo.append(connection)
b = len(cluster2)

print(a * b)
