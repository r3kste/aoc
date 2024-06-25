function part1(input, ptr=1)
    if '(' ∉ input
        return length(input)
    end
    count = 0
    while ptr < length(input)
        if input[ptr] == '('
            close = findnext(isequal(')'), input, ptr)
            marker = input[ptr+1:close-1]
            next, repeat = parse.(Int, split(marker, 'x'))
            return count + next * repeat + part1(input, close + next + 1)
        else
            count += 1
            ptr += 1
        end
    end
    return count
end

function part2(input, ptr=1)
    if '(' ∉ input
        return length(input)
    end
    count = 0
    while ptr < length(input)
        if input[ptr] == '('
            close = findnext(isequal(')'), input, ptr)
            marker = input[ptr+1:close-1]
            next, repeat = parse.(Int, split(marker, 'x'))
            return count + repeat * part2(input[close+1:close+next]) + part2(input, close + next + 1)
        else
            ptr += 1
            count += 1
        end
    end
    return count
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day09.txt")
input = strip(read(input_file, String))
close(input_file)

@time part1_result, part2_result = part1(input), part2(input)
println("Day 09:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
