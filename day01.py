left = []
right = []

with open("inputs/01.txt") as file:
    for line in file.readlines():
        l, r = map(int, line.split("   "))
        left.append(l)
        right.append(r)

left = sorted(left)
right = sorted(right)

distance = similarity = 0

for index, item in enumerate(left):
    distance += abs(item - right[index])
    similarity += right.count(item) * item

with open("outputs/01.txt", "w") as file:
    file.write(f"Part 1: {distance}\nPart 2: {similarity}")
