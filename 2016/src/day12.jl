function part1(input)
    registers = Dict{String,Int}("a" => 0, "b" => 0, "c" => 0, "d" => 0)
    i = 1
    instructions = split(input, "\n")
    while i <= length(instructions)
        words = split(instructions[i])
        if words[1] == "cpy"
            from, to = words[2], words[3]
            if isdigit(from[1])
                registers[to] = parse(Int, from)
            else
                registers[to] = registers[from]
            end
        elseif words[1] == "inc"
            registers[words[2]] += 1
        elseif words[1] == "dec"
            registers[words[2]] -= 1
        elseif words[1] == "jnz"
            check, jump = words[2], words[3]
            if isdigit(check[1])
                check = parse(Int, check)
            else
                check = registers[check]
            end
            if check != 0
                jump = parse(Int, jump)
                i = i + jump - 1
            end
        end
        i += 1
    end

    return registers["a"]
end

function part2(input)
    registers = Dict{String,Int}("a" => 0, "b" => 0, "c" => 1, "d" => 0)
    i = 1
    instructions = split(input, "\n")
    while i <= length(instructions)
        words = split(instructions[i])
        if words[1] == "cpy"
            from, to = words[2], words[3]
            if isdigit(from[1])
                registers[to] = parse(Int, from)
            else
                registers[to] = registers[from]
            end
        elseif words[1] == "inc"
            registers[words[2]] += 1
        elseif words[1] == "dec"
            registers[words[2]] -= 1
        elseif words[1] == "jnz"
            check, jump = words[2], words[3]
            if isdigit(check[1])
                check = parse(Int, check)
            else
                check = registers[check]
            end
            if check != 0
                jump = parse(Int, jump)
                i = i + jump - 1
            end
        end
        i += 1
    end

    return registers["a"]
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day12.txt")
input = strip(read(input_file, String))
close(input_file)

@time part1_result, part2_result = part1(input), part2(input)

println("Day 12:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
