with open("input.txt", "r") as f:
    contents = f.read()

groups = [list(map(set, g.splitlines())) for g in contents.split("\n\n")]

p1 = sum([len(set.union(*g)) for g in groups])
print(f"Part1: {p1}")

p2 = sum([len(set.intersection(*g)) for g in groups])
print(f"Part2: {p2}")
