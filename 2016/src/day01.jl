function part1n2(input::String)
    step = Dict('N' => (0, 1), 'E' => (1, 0), 'S' => (0, -1), 'W' => (-1, 0))
    direction = ['N', 'E', 'S', 'W']
    facing = 1
    X, Y = 0, 0

    part1_result, part2_result = 0, Nothing

    visited = Set{Tuple{Int,Int}}()
    for instruction in split(input, ", ")
        turn, distance = instruction[1], parse(Int, instruction[2:end])
        facing = (turn == 'R') ? facing + 1 : facing - 1

        if facing < 1
            facing = 4
        elseif facing > 4
            facing = 1
        end


        for _ in 1:distance
            X += step[direction[facing]][1]
            Y += step[direction[facing]][2]

            if (X, Y) in visited && part2_result == Nothing
                part2_result = abs(X) + abs(Y)
            else
                push!(visited, (X, Y))
            end
        end
    end

    return (part1_result=abs(X) + abs(Y), part2_result)
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day01.txt", "r")
input = readlines(input_file)[1]
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 01:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
