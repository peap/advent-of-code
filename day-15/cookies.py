from itertools import combinations, permutations
import re

REGEX = re.compile(
    r'^(\w+): capacity ([\-\d]+), durability ([\-\d]+), flavor ([\-\d]+), '
    r'texture ([\-\d]+), calories ([\-\d]+)$'
)
TOTAL_TSP = 100

INGREDIENTS = {}

with open('ingredients.txt', 'r') as f:
    for line in f.readlines():
        match = REGEX.match(line.strip())
        ingredient, cap, dur, flav, tex, cal = match.groups()
        INGREDIENTS[ingredient] = {
            'capacity': int(cap),
            'durability': int(dur),
            'flavor': int(flav),
            'texture': int(tex),
            'calories': int(cal),
        }

ingredient_names = INGREDIENTS.keys()

def get_score(ingredients, amounts):
    calories = 0
    score = {
        'capacity': 0,
        'durability': 0,
        'flavor': 0,
        'texture': 0,
    }
    for ingredient, amount in zip(ingredients, amounts):
        if calories > 500:
            break
        attrs = INGREDIENTS[ingredient]
        for attr, val in attrs.iteritems():
            if attr == 'calories':
                calories += amount * val
                continue
            score[attr] += amount * val
    for attr, val in score.iteritems():
        if val < 0:
            score[attr] = 0
    if calories == 500:
        return reduce(lambda n, t: n * t, score.values(), 1) 
    else:
        return 0

# build ingredient combinations
combos = []
for x in range(len(ingredient_names)):
    combos += list(combinations(ingredient_names, x + 1))

# build amount distribution combinations/permutations
amount_range = range(1, TOTAL_TSP + 1)
amounts = {}
for x in range(len(ingredient_names)):
    n_ing = x + 1
    amounts[n_ing] = set()
    amount_combos = set(combinations(n_ing * amount_range, n_ing))
    for combo in amount_combos:
        if sum(combo) == TOTAL_TSP:
            for perm in set(permutations(combo)):
                amounts[n_ing].add(perm)

# try each combo
scores = []
for ing_list in combos:
    for amount_distribution in amounts[len(ing_list)]:
        score = get_score(ing_list, amount_distribution)
        scores.append((ing_list, amount_distribution, score))

# print top ten
for ing_list, amount_distribution, score in sorted(scores, key=lambda x: -1 * x[2])[:10]:
    print(
        '{0} <-- {1} ({2})'.format(
            score,
            ', '.join(ing_list),
            ', '.join([str(x) for x in amount_distribution]),
        )
    )
