function part1(input)
    input = bitstring(parse(Int, input))
    input = input[findfirst(c -> c == '1', input):end][2:end] * "1"
    return parse(Int, input, base=2)
end

function part2(input)
    input = parse(Int, input)
    i = 1
    while i * 3 < input
        i *= 3
    end
    return input - i
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day19.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 19:")
@time part1_result = part1(input)
println("Part 1: $part1_result")
@time part2_result = part2(input)
println("Part 2: $part2_result")
