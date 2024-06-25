using Combinatorics

struct Node
    x::Int
    y::Int
    neighbors::Vector{Tuple{Int,Int}}
end


function part1n2(input)
    grid = []
    steps = [(0, -1), (0, 1), (-1, 0), (1, 0)]

    for line in split(input, "\n")
        push!(grid, [c for c in line])
    end

    nodes = Dict()
    num_pos = Dict()
    rows = length(grid)
    cols = length(grid[1])

    for i in 2:rows-1
        for j in 2:cols-1
            if grid[i][j] == '#'
                continue
            end
            if isdigit(grid[i][j])
                num_pos[parse(Int, grid[i][j])] = (i, j)
            end
            nodes[(i, j)] = Node(i, j, [])

            for step in steps
                next_i = i + step[1]
                next_j = j + step[2]

                if grid[next_i][next_j] == '#'
                    continue
                end

                push!(nodes[(i, j)].neighbors, (next_i, next_j))
            end
        end
    end

    distances = Dict()
    for num in keys(num_pos)
        for num2 in keys(num_pos)
            if num == num2 || haskey(distances, (num, num2))
                continue
            end

            seen = Set()
            queue = [(0, num_pos[num])]
            while !isempty(queue)
                dist, cur = popfirst!(queue)
                if cur in seen
                    continue
                end

                push!(seen, cur)

                if cur == num_pos[num2]
                    distances[(num, num2)] = dist
                    distances[(num2, num)] = dist
                    break
                end

                for neighbor in nodes[cur].neighbors
                    new_dist = dist + abs(neighbor[1] - cur[1]) + abs(neighbor[2] - cur[2])
                    push!(queue, (new_dist, neighbor))
                end
            end
        end
    end

    part1_result, part2_result = 1000000, 1000000
    for path in permutations([1, 2, 3, 4, 5, 6, 7])
        dist = distances[(0, path[1])]
        for i in 1:length(path)-1
            dist += distances[(path[i], path[i+1])]
        end
        part1_result = min(part1_result, dist)
        part2_result = min(part2_result, dist + distances[(path[end], 0)])
    end

    return (part1_result, part2_result)
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day24.txt")
input = strip(read(input_file, String))
close(input_file)

println("Day 24:")
@time part1_result, part2_result = part1n2(input)
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
