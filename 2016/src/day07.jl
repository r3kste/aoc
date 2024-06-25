function part1n2(input)
    part1_result = 0
    part2_result = 0

    for ip in split(input, '\n')
        hypnet = false

        part1_valid = Nothing
        for (i, c) in enumerate(ip)
            if c == '['
                hypnet = true
            elseif c == ']'
                hypnet = false
            else
                if i + 3 <= length(ip)
                    if ip[i] == ip[i+3] && ip[i+1] == ip[i+2] && ip[i] != ip[i+1]
                        if hypnet
                            part1_valid = false
                            break
                        else
                            part1_valid = true
                        end
                    end
                end
            end
        end

        hypnet = false

        aba = []
        bab = []
        for (j, c) in enumerate(ip)
            if c == '['
                hypnet = true
            elseif c == ']'
                hypnet = false
            else
                if j + 2 <= length(ip)
                    if ip[j] == ip[j+2] && ip[j] != ip[j+1]
                        if hypnet
                            push!(bab, (ip[j:j+2]))
                        else
                            push!(aba, (ip[j:j+2]))
                        end
                    end
                end
            end
        end

        if part1_valid == true
            part1_result += 1
        end

        for p in aba
            pq = p[2] * p[1] * p[2]
            if any(q == pq for q in bab)
                part2_result += 1
                break
            end
        end
    end

    return (part1_result, part2_result)
end

input_file = open(dirname(@__FILE__) * "/../.inputs/day07.txt", "r")
input = strip(read(input_file, String))
close(input_file)

@time part1_result, part2_result = part1n2(input)

println("Day 07:")
println("Part 1: ", part1_result)
println("Part 2: ", part2_result)
