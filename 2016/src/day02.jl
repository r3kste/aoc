function part1n2(input::String)
    part_1_keypad = [1 2 3; 4 5 6; 7 8 9]
    part_2_keypad = [0 0 1 0 0; 0 2 3 4 0; 5 6 7 8 9; 0 'A' 'B' 'C' 0; 0 0 'D' 0 0]

    step = Dict('U' => (0, -1), 'D' => (0, 1), 'L' => (-1, 0), 'R' => (1, 0))

    moves = split(input, "\n")
    part_1_pos = (2, 2)
    part_2_pos = (1, 3)
    part1_code, part2_code = "", ""

    for move in moves
        if move == ""
            continue
        end

        for c in move
            part_1_pos = Tuple(clamp(r, 1, 3) for r in (part_1_pos .+ step[c]))

            x, y = part_2_pos = (part_2_pos .+ step[c])
            if step[c][1] == 0
                part_2_pos = (x, clamp(y, 1 + abs(3 - x), 5 - abs(3 - x)))
            else
                part_2_pos = (clamp(x, 1 + abs(3 - y), 5 - abs(3 - y)), y)
            end
        end

        part1_code *= string(part_1_keypad[part_1_pos[2], part_1_pos[1]])
        part2_code *= string(part_2_keypad[part_2_pos[2], part_2_pos[1]])
    end
    return part1_code, part2_code
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day02.txt", "r")
input = read(input_file, String)
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 02:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
