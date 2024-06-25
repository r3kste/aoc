function part1n2(input)
    grid = [[c == '.' for c in input]]

    part1_result = 0
    while length(grid) < 400000
        row = grid[end]
        next_row = []

        for i in eachindex(row)
            left = i == 1 ? true : row[i-1]
            center = row[i]
            right = i == length(row) ? true : row[i+1]

            trapped = (left && center && !right) ||
                      (!left && center && right) ||
                      (left && !center && !right) ||
                      (!left && !center && right)
            push!(next_row, !trapped)
        end

        push!(grid, next_row)

        if length(grid) == 40
            part1_result = sum(sum(row) for row in grid)
        end
    end

    return part1_result, sum(sum(row) for row in grid)
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day18.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 18:")
@time part1_result, part2_result = part1n2(input)
println("Part 1: $part1_result")
println("Part 2: $part2_result")
