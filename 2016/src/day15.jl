function part1(input)
    discs = []
    for (i, disc) in enumerate(split(input, '\n'))
        words = split(disc, ' ')
        npos, pos = parse(Int, words[4]), parse(Int, words[12][1:end-1])
        push!(discs, (pos + i, npos))
    end

    t = 0
    while true
        if all((disc[1] + t) % disc[2] == 0 for (i, disc) in enumerate(discs))
            return t
        end
        t += 1
    end
end

function part2(input)
    discs = []
    for (i, disc) in enumerate(split(input, '\n'))
        words = split(disc, ' ')
        npos, pos = parse(Int, words[4]), parse(Int, words[12][1:end-1])
        push!(discs, (pos + i, npos))
    end
    push!(discs, (length(discs) + 1, 11))

    t = 0
    while true
        if all((disc[1] + t) % disc[2] == 0 for (i, disc) in enumerate(discs))
            return t
        end
        t += 1
    end
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day15.txt")
input = strip(read(input_file, String))
close(input_file)

println("Day 15:")
@time part1_result = part1(input)
println("Part 1: ", part1_result)

@time part2_result = part2(input)
println("Part 2: ", part2_result)
