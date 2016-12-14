from collections import Counter

with open('input.txt', 'r') as f:
    data = f.read().strip()

# part 1

x = 0
y = 0

presents = Counter([(0, 0)])

for instruction in data:
    if instruction == '<':
        x -= 1
    if instruction == '>':
        x += 1
    if instruction == '^':
        y += 1
    if instruction == 'v':
        y -= 1

    house = (x, y)
    presents[house] += 1


print(len(presents))


# part 2

x = {True: 0, False: 0}
y = {True: 0, False: 0}

presents = Counter([(0, 0), (0, 0)])

robo = False
for instruction in data:
    if instruction == '<':
        x[robo] -= 1
    if instruction == '>':
        x[robo] += 1
    if instruction == '^':
        y[robo] += 1
    if instruction == 'v':
        y[robo] -= 1

    if instruction not in ['v', '^', '<', '>']:
        raise ValueError('oh no!')

    house = (x[robo], y[robo])
    presents[house] += 1

    robo = not robo


print(len(presents))
