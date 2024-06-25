using Combinatorics

function process_instruction(str, instruction)
    str = deepcopy(str)
    words = split(instruction)
    if words[1] == "swap"
        if words[2] == "position"
            x, y = parse.(Int, [words[3], words[6]])
            x, y = x + 1, y + 1
            str[x], str[y] = str[y], str[x]
        elseif words[2] == "letter"
            x, y = words[3][1], words[6][1]
            x, y = findfirst([c == x for c in str]), findfirst([c == y for c in str])
            str[x], str[y] = str[y], str[x]
        end
    elseif words[1] == "rotate"
        if words[2] == "left"
            x = parse(Int, words[3])
            x = x % length(str)
            str = cat(str[x+1:end], str[1:x], dims=1)
        elseif words[2] == "right"
            x = parse(Int, words[3])
            x = x % length(str)
            str = cat(str[end-x+1:end], str[1:end-x], dims=1)
        elseif words[2] == "based"
            x = findfirst(isequal(words[7][1]), str)
            x += x >= 5 ? 1 : 0
            x = x % length(str)
            str = cat(str[end-x+1:end], str[1:end-x], dims=1)
        end
    elseif words[1] == "reverse"
        x, y = parse.(Int, [words[3], words[5]])
        x, y = x + 1, y + 1
        str = cat(str[1:x-1], reverse(str[x:y]), str[y+1:end], dims=1)
    elseif words[1] == "move"
        x, y = parse.(Int, [words[3], words[6]])
        x, y = x + 1, y + 1
        c = str[x]
        str = cat(str[1:x-1], str[x+1:end], dims=1)
        str = cat(str[1:y-1], [c], str[y:end], dims=1)
    end

    return str
end

function part1(input, str="abcdefgh")
    str = [c for c in str]
    for instruction in split(input, "\n")
        str = process_instruction(str, instruction)
    end

    return join(str)
end

function part2(input, str="fbgdceah")
    for perm in permutations([c for c in "bacdefgh"])
        perm = join(perm)
        if part1(input, perm) == str
            return perm
        end
    end

    return -1
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day21.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 21:")
@time part1_result = part1(input)
println("Part 1: $part1_result")
@time part2_result = part2(input)
println("Part 2: $part2_result")
