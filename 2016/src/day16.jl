function part1(input)
    data = [c for c in input]
    LENGTH = 272

    while length(data) < LENGTH
        data = cat(data, ['0'], reverse([c == '0' ? '1' : '0' for c in data]), dims=1)
    end

    checksum = data[1:LENGTH]
    while length(checksum) % 2 == 0
        new_checksum = [checksum[i] == checksum[i+1] ? '1' : '0' for i in 1:2:length(checksum)]
        checksum = deepcopy(new_checksum)
    end

    return join(checksum)
end

function part2(input)
    data = [c for c in input]
    LENGTH = 35651584

    while length(data) < LENGTH
        data = cat(data, ['0'], reverse([c == '0' ? '1' : '0' for c in data]), dims=1)
    end

    checksum = data[1:LENGTH]
    while length(checksum) % 2 == 0
        new_checksum = [checksum[i] == checksum[i+1] ? '1' : '0' for i in 1:2:length(checksum)]
        checksum = deepcopy(new_checksum)
    end

    return join(checksum)
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day16.txt")
input = strip(read(input_file, String))
close(input_file)

println("Day 16:")
@time part1_result = part1(input)
println("Part 1: ", part1_result)

@time part2_result = part2(input)
println("Part 2: ", part2_result)
