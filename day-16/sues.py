import re

TOP_LEVEL_RE = re.compile(r'^Sue (\d+): (.*)')
ATTR_RE = re.compile(r'(\w+): (\d+)')

SUES = {}

EXPECTED_SUE = {
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1,
}

with open('input.txt', 'r') as f:
    for line in f.readlines():
        main_match = TOP_LEVEL_RE.match(line)
        sue, attrs = main_match.groups()
        SUES[sue] = {}
        for attr, count in ATTR_RE.findall(attrs):
            SUES[sue][attr] = int(count)

# part 1
for sue, attrs in SUES.iteritems():
    matches = False
    for attr, count in attrs.iteritems():
        matches = count == EXPECTED_SUE[attr]
        if not matches:
            break
    if matches:
        print('exact match: Sue {0}'.format(sue))

# part 2
GREATER_THAN = ('cats', 'trees')
FEWER_THAN = ('pomeranians', 'goldfish')
for sue, attrs in SUES.iteritems():
    matches = False
    for attr, count in attrs.iteritems():
        expected = EXPECTED_SUE[attr]
        if attr in GREATER_THAN:
            matches = count > expected
        elif attr in FEWER_THAN:
            matches = count < expected
        else:
            matches = count == EXPECTED_SUE[attr]
        if not matches:
            break
    if matches:
        print('exact match: Sue {0}'.format(sue))
