using MD5

function part1(input)
    part1_passwd = ""
    i = 0
    while true
        hash = md5(input * string(i))
        hash = join((string(byte, base=16, pad=2) for byte in hash))
        if startswith(hash, "00000")
            part1_passwd *= hash[6]
            print(hash[6])
            if length(part1_passwd) == 8
                break
            end
        end
        i += 1
    end
    println()
end

function part2(input)
    part2_passwd = fill(' ', 8)
    i = 0
    while true
        hash = md5(input * string(i))
        hash = join((string(byte, base=16, pad=2) for byte in hash))

        if startswith(hash, "00000")
            pos = hash[6]
            if '0' <= pos <= '7'
                pos = parse(Int, pos) + 1
                if part2_passwd[pos] == ' '
                    part2_passwd[pos] = hash[7]
                    print("\rPart 2: ", join(part2_passwd))
                    if all(c != ' ' for c in part2_passwd)
                        break
                    end
                end
            end
        end
        i += 1
    end
    println()
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day05.txt", "r")
input = strip(read(input_file, String))
close(input_file)

println("Day 05:")
print("Part 1: ")
@time part1(input)
print("Part 2: ")
@time part2(input)
