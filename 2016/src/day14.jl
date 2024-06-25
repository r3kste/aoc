using MD5

function part1(input)
    threes = Dict{Int,Char}()
    fives = Dict{Int,Set{Char}}()

    i = 0
    while i < 20000
        hash = md5(input * string(i))
        hash = join((string(byte, base=16, pad=2) for byte in hash))

        for j in 1:length(hash)-2
            if hash[j] == hash[j+1] == hash[j+2]
                threes[i] = hash[j]
                break
            end
        end

        for j in 1:length(hash)-4
            if hash[j] == hash[j+1] == hash[j+2] == hash[j+3] == hash[j+4]
                if !haskey(fives, i)
                    fives[i] = Set{Char}()
                end
                push!(fives[i], hash[j])
            end
        end

        i += 1
    end

    key = []
    for three in sort(collect(keys(threes)))
        for i in three+1:three+1001
            if haskey(fives, i) && threes[three] in fives[i]
                push!(key, three)
                break
            end
        end
    end

    return key[64]
end

function part2(input)
    threes = Dict{Int,Char}()
    fives = Dict{Int,Set{Char}}()

    i = 0
    while i < 30000
        hash = input * string(i)
        for _ in 1:2017
            hash = md5(hash)
            hash = join((string(byte, base=16, pad=2) for byte in hash))
        end

        for j in 1:length(hash)-2
            if hash[j] == hash[j+1] == hash[j+2]
                threes[i] = hash[j]
                break
            end
        end 

        for j in 1:length(hash)-4
            if hash[j] == hash[j+1] == hash[j+2] == hash[j+3] == hash[j+4]
                if !haskey(fives, i)
                    fives[i] = Set{Char}()
                end
                push!(fives[i], hash[j])
            end
        end

        i += 1
    end

    key = []
    for three in sort(collect(keys(threes)))
        for i in three+1:three+1001
            if haskey(fives, i) && threes[three] in fives[i]
                push!(key, three)
                break
            end
        end
    end

    return key[64]
end


input_file = open(dirname(@__FILE__) * "/../.inputs/day14.txt")
input = strip(read(input_file, String))
close(input_file)

println("Day 14:")
@time part1_result = part1(input)
println("Part 1: ", part1_result)

@time part2_result = part2(input)
println("Part 2: ", part2_result)
