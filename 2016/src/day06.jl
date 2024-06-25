function part1n2(input)
    freq = Dict{Int,Dict{Char,Int}}(i => Dict{Char,Int}(c => 0 for c in 'a':'z') for i in 1:8)

    for message in split(input, '\n')
        for (i, c) in enumerate(message)
            freq[i][c] += 1
        end
    end

    part1_result = ""
    part2_result = ""
    for i in 1:8
        part1_result *= string(argmax(freq[i]))
        part2_result *= string(argmin(freq[i]))
    end

    return (part1_result, part2_result)
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day06.txt", "r")
input = strip(read(input_file, String))
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 06:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
