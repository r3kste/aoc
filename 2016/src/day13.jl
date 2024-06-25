function part1(grid)
    visited = Set([(1, 1)])
    queue = [(0, (1, 1))]

    while !isempty(queue)
        steps, (x, y) = popfirst!(queue)
        if x == 31 && y == 39
            return steps
        end

        for (dx, dy) in ((0, 1), (0, -1), (1, 0), (-1, 0))
            new_x, new_y = x + dx, y + dy
            if new_x >= 0 && new_y >= 0 && count_ones(grid[new_x, new_y]) % 2 == 0 && !((new_x, new_y) in visited)
                push!(queue, (steps + 1, (new_x, new_y)))
                push!(visited, (new_x, new_y))
            end
        end
    end
end

function part2(grid)
    visited = Set([(1, 1)])
    queue = [(0, 50, (1, 1))]

    while !isempty(queue)
        steps, rem, (x, y) = popfirst!(queue)
        if rem == 0
            continue
        end

        for (dx, dy) in ((0, 1), (0, -1), (1, 0), (-1, 0))
            new_x, new_y = x + dx, y + dy
            if new_x >= 0 && new_y >= 0 && count_ones(grid[new_x, new_y]) % 2 == 0 && !((new_x, new_y) in visited)
                push!(queue, (steps + 1, rem - 1, (new_x, new_y)))
                push!(visited, (new_x, new_y))
            end
        end
    end

    return length(visited)
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day13.txt")
input = strip(read(input_file, String))
close(input_file)

grid = Dict((x, y) => (x * x + 3 * x + 2 * x * y + y + y * y) + parse(Int, input) for x in 0:50, y in 0:50)

@time part1_result, part2_result = part1(grid), part2(grid)

println("Day 13:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
