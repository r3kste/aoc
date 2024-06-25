function part1n2(input::String)
    rooms = split(input, "\n")

    part1_result, part2_result = 0, Nothing
    for room in rooms
        if room == ""
            continue
        end

        parts = split(room, "-")
        name = join(parts[1:end-1])
        id = parse(Int, parts[end][1:3])
        checksum = parts[end][5:end-1]

        counts = Dict{Char,Int}()
        for c in name
            if haskey(counts, c)
                counts[c] += 1
            else
                counts[c] = 1
            end
        end

        sorted_counts = sort(collect(counts), by=x -> (-x[2], x[1]))
        calculated_checksum = join([x[1] for x in sorted_counts[1:5]])

        if calculated_checksum == checksum
            part1_result += id
            if part2_result == Nothing
                shift = id % 26
                decrypted = ""
                for c in name
                    decrypted *= Char((Int(c) - 97 + shift) % 26 + 97)
                end

                if contains(decrypted, "north")
                    part2_result = id
                end
            end
        end
    end

    return part1_result, part2_result
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day04.txt", "r")
input = read(input_file, String)
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 04:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
