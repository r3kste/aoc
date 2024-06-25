function part1(instructions)
    registers = Dict{String,Int}("a" => 7, "b" => 0, "c" => 0, "d" => 0)
    i = 1
    while i <= length(instructions)
        words = instructions[i]
        if words[1] == "incby"
            # incby is a custom instruction
            # incby a bc
            # is equivalent to
            # a += b * c
            prod = 1
            for c in words[3]
                prod *= registers[string(c)]
            end
            registers[words[2]] += prod
        elseif words[1] == "tgl"
            instruction = i + registers[words[2]]
            if instruction > 0 && instruction <= length(instructions)
                words = instructions[instruction]
                if length(words) == 2
                    if words[1] == "inc"
                        instructions[instruction] = ["dec", words[2]]
                    else
                        instructions[instruction] = ["inc", words[2]]
                    end
                elseif length(words) == 3
                    if words[1] == "jnz"
                        instructions[instruction] = ["cpy", words[2], words[3]]
                    else
                        instructions[instruction] = ["jnz", words[2], words[3]]
                    end
                end
            end
        elseif words[1] == "cpy"
            from, to = words[2], words[3]
            if !(to in keys(registers))
                i += 1
                continue
            end
            if isdigit(from[end])
                registers[to] = parse(Int, from)
            else
                registers[to] = registers[from]
            end
        elseif words[1] == "inc"
            if !(words[2] in keys(registers))
                i += 1
                continue
            end
            registers[words[2]] += 1
        elseif words[1] == "dec"
            if !(words[2] in keys(registers))
                i += 1
                continue
            end
            registers[words[2]] -= 1
        elseif words[1] == "jnz"
            check, jump = words[2], words[3]
            if isdigit(check[end])
                check = parse(Int, check)
            else
                check = registers[check]
            end
            if isdigit(jump[end])
                jump = parse(Int, jump)
            else
                jump = registers[jump]
            end
            if check != 0
                i = i + jump - 1
            end
        end
        i += 1
    end

    return registers["a"]
end

function part2(instructions)
    registers = Dict{String,Int}("a" => 12, "b" => 0, "c" => 0, "d" => 0)
    i = 1
    while i <= length(instructions)
        words = instructions[i]
        if words[1] == "incby"
            prod = 1
            for c in words[3]
                prod *= registers[string(c)]
            end
            registers[words[2]] += prod
        elseif words[1] == "tgl"
            instruction = i + registers[words[2]]
            if instruction > 0 && instruction <= length(instructions)
                words = instructions[instruction]
                if length(words) == 2
                    if words[1] == "inc"
                        instructions[instruction] = ["dec", words[2]]
                    else
                        instructions[instruction] = ["inc", words[2]]
                    end
                elseif length(words) == 3
                    if words[1] == "jnz"
                        instructions[instruction] = ["cpy", words[2], words[3]]
                    else
                        instructions[instruction] = ["jnz", words[2], words[3]]
                    end
                end
            end
        elseif words[1] == "cpy"
            from, to = words[2], words[3]
            if !(to in keys(registers))
                i += 1
                continue
            end
            if isdigit(from[end])
                registers[to] = parse(Int, from)
            else
                registers[to] = registers[from]
            end
        elseif words[1] == "inc"
            if !(words[2] in keys(registers))
                i += 1
                continue
            end
            registers[words[2]] += 1
        elseif words[1] == "dec"
            if !(words[2] in keys(registers))
                i += 1
                continue
            end
            registers[words[2]] -= 1
        elseif words[1] == "jnz"
            check, jump = words[2], words[3]
            if isdigit(check[end])
                check = parse(Int, check)
            else
                check = registers[check]
            end
            if isdigit(jump[end])
                jump = parse(Int, jump)
            else
                jump = registers[jump]
            end
            if check != 0
                i = i + jump - 1
            end
        end
        i += 1
    end

    return registers["a"]
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day23_2.txt")
input = strip(read(input_file, String))
close(input_file)

instructions = split(input, "\n")
instructions = [split(instruction) for instruction in instructions]

println("Day 23:")
@time part1_result = part1(deepcopy(instructions))
println("Part 1: ", part1_result)
@time part2_result = part2(deepcopy(instructions))
println("Part 2: ", part2_result)
