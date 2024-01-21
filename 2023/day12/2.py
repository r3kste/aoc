import os

cache = {}


def f(cfg, nums):
    if cfg == "":
        return 1 if nums == () else 0

    if nums == ():
        return 0 if "#" in cfg else 1

    key = (cfg, nums)

    if key in cache:
        return cache[key]

    result = 0

    if cfg[0] in ".?":
        result += f(cfg[1:], nums)

    if cfg[0] in "#?":
        if nums[0] <= len(cfg) and "." not in cfg[:nums[0]] and (nums[0] == len(cfg) or cfg[nums[0]] != "#"):
            result += f(cfg[nums[0] + 1:], nums[1:])

    cache[key] = result
    return result


ans = 0

for line in open(os.path.dirname(__file__) + "/input.txt", "r"):
    cfg, nums = line.split()
    nums = tuple(map(int, nums.split(",")))

    cfg = "?".join([cfg] * 5)
    nums *= 5

    ans += f(cfg, nums)

print(ans)
