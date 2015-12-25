from itertools import combinations

CONTAINERS = [int(x) for x in """
33
14
18
20
45
35
16
35
1
13
18
13
50
44
48
6
24
41
30
42
""".strip().split('\n')
]

TARGET_SIZE = 150

valid_combos = []
for n in range(1, len(CONTAINERS) + 1):
    for combo in combinations(CONTAINERS, n):
        if sum(combo) == TARGET_SIZE:
            valid_combos.append(combo)

print('total combos: {0}'.format(len(valid_combos)))

min_length = min([len(x) for x in valid_combos])
print(len(filter(lambda x: len(x) == min_length, valid_combos)))
