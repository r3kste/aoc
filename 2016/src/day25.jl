function part1(instructions)
    for value in 0:256
        output = ""
        registers = Dict{String,Int}("a" => value, "b" => 0, "c" => 0, "d" => 0)
        i = 1
        while i <= length(instructions)
            words = instructions[i]
            if words[1] == "out"
                output *= string(registers[words[2]])
                if startswith(output, "1") || contains(output, "11") || contains(output, "00")
                    break
                end
                if length(output) == 8
                    break
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
        if output == "01010101"
            return value
        end
    end
    return -1
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day25.txt")
input = strip(read(input_file, String))
close(input_file)

instructions = split(input, "\n")
instructions = [split(instruction) for instruction in instructions]

println("Day 25:")
@time part1_result = part1(instructions)
println("Part 1: ", part1_result)
