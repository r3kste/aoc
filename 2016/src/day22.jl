struct Node
    x::Int
    y::Int
    size::Int
    used::Int
    avail::Int
end

function part1(nodes)
    part1_result = 0

    for i in eachindex(nodes)
        for j in eachindex(nodes)
            if i == j
                continue
            end

            used = nodes[i].used
            avail = nodes[j].avail
            if used != 0 && used <= avail
                part1_result += 1
            end
        end
    end

    return part1_result
end

function part2(nodes)
    rows = maximum([node.y for node in nodes]) + 1
    cols = maximum([node.x for node in nodes]) + 1

    grid = Dict()
    for node in nodes
        grid[(node.x, node.y)] = node
    end

    G = grid[(cols - 1, 0)]
    empty = grid[(0, 0)]
    walls = []

    for i in 0:rows-1
        for j in 0:cols-1
            node = grid[(j, i)]
            char = '.'
            if node.used == 0
                empty = deepcopy(node)
                char = '_'
            elseif node == G
                char = 'G'
            elseif node.x == 0 && node.y == 0
                char = 'E'
            elseif node.used > empty.size
                char = '#'
                push!(walls, node)
            end
            # print(char)
        end
        # println()
    end

    min_wall = minimum(wall.x for wall in walls)
    threshold = min_wall - 1

    part2_result = (empty.x - threshold)
    part2_result += empty.y
    part2_result += G.x - threshold
    part2_result += 5 * (cols - 2)

    return part2_result
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day22.txt", "r")
input = strip(read(input_file, String))
close(input_file)

nodes = []
for node in split(input, "\n")[3:end]
    node, size, used, avail, _ = split(node)
    size, used, avail = [w[1:end-1] for w in (size, used, avail)]
    size, used, avail = parse.(Int, (size, used, avail))

    node = split(node, "-")
    x, y = parse.(Int, [node[2][2:end], node[3][2:end]])

    push!(nodes, Node(x, y, size, used, avail))
end

println("Day 22:")
@time part1_result = part1(nodes)
println("Part 1: $part1_result")
@time part2_result = part2(nodes)
println("Part 2: $part2_result")
