function part1n2(input)
    blacklist = []
    for ranges in split(input, "\n")
        l, r = parse.(Int, split(ranges, "-"))
        push!(blacklist, (l, r))
    end
    sort!(blacklist)

    i = 2
    while i <= length(blacklist)
        (l1, r1) = blacklist[i-1]
        (l2, r2) = blacklist[i]

        if l2 <= r1 + 1
            blacklist[i-1] = (l1, max(r2, r1))
            deleteat!(blacklist, i)
            i -= 1
        end

        i += 1
    end

    part1_result = blacklist[1][2] + 1

    part2_result = 0
    for i in eachindex(blacklist)[2:end]
        (_, r1) = blacklist[i-1]
        (l2, _) = blacklist[i]

        part2_result += l2 - r1 - 1
    end

    return (part1_result, part2_result)
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day20.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 20:")
@time part1_result, part2_result = part1n2(input)
println("Part 1: $part1_result")
println("Part 2: $part2_result")
