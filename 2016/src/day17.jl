using MD5

function part1(input)
    queue = [((0, 0), input)]

    while !isempty(queue)
        ((x, y), path) = popfirst!(queue)

        if x == 3 && y == 3
            return path[length(input)+1:end]
        end

        hash = md5(path)
        hash = join((string(byte, base=16, pad=2) for byte in hash[1:2]))
        hash = [(c in 'b':'f') for c in hash]

        if y > 0 && hash[1]
            push!(queue, ((x, y - 1), path * "U"))
        end
        if y < 3 && hash[2]
            push!(queue, ((x, y + 1), path * "D"))
        end
        if x > 0 && hash[3]
            push!(queue, ((x - 1, y), path * "L"))
        end
        if x < 3 && hash[4]
            push!(queue, ((x + 1, y), path * "R"))
        end
    end
    return -1
end

function part2(input)
    queue = [((0, 0), input)]

    part2_result = 0
    while !isempty(queue)
        ((x, y), path) = popfirst!(queue)

        if x == 3 && y == 3
            part2_result = max(part2_result, length(path) - length(input))
            continue
        end

        hash = md5(path)
        hash = join((string(byte, base=16, pad=2) for byte in hash[1:2]))
        hash = [(c in 'b':'f') for c in hash]

        if y > 0 && hash[1]
            push!(queue, ((x, y - 1), path * "U"))
        end
        if y < 3 && hash[2]
            push!(queue, ((x, y + 1), path * "D"))
        end
        if x > 0 && hash[3]
            push!(queue, ((x - 1, y), path * "L"))
        end
        if x < 3 && hash[4]
            push!(queue, ((x + 1, y), path * "R"))
        end
    end
    return part2_result
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day17.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 17:")
@time part1_result = part1(input)
println("Part 1: $part1_result")
@time part2_result = part2(input)
println("Part 2: $part2_result")
