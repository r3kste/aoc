function part1n2(input)
    grid = [['.' for _ in 1:50] for _ in 1:6]

    for instruction in split(input, '\n')
        words = split(instruction, ' ')
        type = words[1]
        if type == "rect"
            cols, rows = split(words[2], 'x')
            cols, rows = parse(Int, cols), parse(Int, rows)
            for i in 1:rows
                for j in 1:cols
                    grid[i][j] = '#'
                end
            end
        elseif type == "rotate"
            axis = words[3]
            axis, pos = split(axis, '=')
            pos = parse(Int, pos)
            shift = parse(Int, words[5])

            new = []
            if axis == "x"
                for row in 1:6
                    push!(new, grid[row][pos+1])
                end
                circshift!(new, shift)
                for row in 1:6
                    grid[row][pos+1] = new[row]
                end
            elseif axis == "y"
                for col in 1:50
                    push!(new, grid[pos+1][col])
                end
                circshift!(new, shift)
                for col in 1:50
                    grid[pos+1][col] = new[col]
                end
            end
        end
    end
    part1_result = 0
    for row in grid
        for e in row
            if e == '#'
                part1_result += 1
            end
        end
    end

    part2_result = grid
    return part1_result, part2_result
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day08.txt")
input = strip(read(input_file, String))
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 08:")
println("Part 1: ", part1_result)
println("Part 2:")
for row in part2_result
    for e in row
        print(e * ' ')
    end
    println()
end
