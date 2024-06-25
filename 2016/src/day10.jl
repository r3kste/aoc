function part1(input)
    chips, instructions, current = input

    while !isempty(current)
        bot = pop!(current)
        if bot < 0
            continue
        end

        low, high = instructions[bot]
        low_value, high_value = sort(collect(chips[bot]))
        if low_value == 17 && high_value == 61
            return bot
        end

        chips[low] = get!(chips, low, Set{Int}())
        chips[high] = get!(chips, high, Set{Int}())
        push!(chips[low], low_value)
        push!(chips[high], high_value)
        if length(chips[low]) == 2
            push!(current, low)
        end
        if length(chips[high]) == 2
            push!(current, high)
        end
    end

    return 0
end

function part2(input)
    chips, instructions, current = input

    while !isempty(current)
        bot = pop!(current)
        if bot < 0
            continue
        end

        low, high = instructions[bot]
        low_value, high_value = sort(collect(chips[bot]))

        chips[low] = get!(chips, low, Set{Int}())
        chips[high] = get!(chips, high, Set{Int}())
        push!(chips[low], low_value)
        push!(chips[high], high_value)
        if length(chips[low]) == 2
            push!(current, low)
        end
        if length(chips[high]) == 2
            push!(current, high)
        end
    end

    part2_result = 1
    for i in 1:3
        if haskey(chips, -i)
            for value in chips[-i]
                part2_result *= value
            end
        end
    end

    return part2_result
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day10.txt")
input = strip(read(input_file, String))
close(input_file)

chips = Dict{Int,Set{Int}}()
instructions = Dict{Int,Tuple{Int,Int}}()
current = []

for instruction in split(input, "\n")
    words = split(instruction, " ")
    if words[1] == "value"
        value = parse(Int, words[2])
        target = parse(Int, words[6])
        push!(get!(chips, target, Set{Int}()), value)
        if length(chips[target]) == 2
            push!(current, target)
        end
    elseif words[1] == "bot"
        bot = parse(Int, words[2])
        low = parse(Int, words[7])
        if words[6] == "output"
            low = -low - 1
        end

        high = parse(Int, words[12])
        if words[11] == "output"
            high = -high - 1
        end

        instructions[bot] = (low, high)
    end
end

input1 = (deepcopy(chips), deepcopy(instructions), deepcopy(current))
input2 = (deepcopy(chips), deepcopy(instructions), deepcopy(current))
@time part1_result, part2_result = part1(input1), part2(input2)

println("Day 10:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
