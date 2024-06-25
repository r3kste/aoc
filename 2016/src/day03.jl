function part1(triangles)
    result = 0
    for triangle in triangles
        if check_sides(triangle)
            result += 1
        end
    end

    return result
end

function part2(triangles)
    result = 0

    for i in 1:3
        for j in 1:3:length(triangles)
            sides = [triangles[j][i], triangles[j+1][i], triangles[j+2][i]]
            if check_sides(sides)
                result += 1
            end
        end
    end

    return result
end

function check_sides(sides)
    sides = sort(sides)
    return sides[1] + sides[2] > sides[3]
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day03.txt", "r")
input = read(input_file, String)
close(input_file)

triangles = []
for line in split(input, "\n")
    if line == ""
        continue
    end

    sides = [parse(Int, x) for x in split(strip(line))]
    push!(triangles, sides)
end

@time part1_result, part2_result = part1(triangles), part2(triangles)

println("Day 03:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)