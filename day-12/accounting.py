import json

with open('input.json', 'r') as f:
    data = json.load(f)

def sum_of_list(items, ignore_red=False):
    total = 0
    for item in items:
        if isinstance(item, list):
            total += sum_of_list(item, ignore_red=ignore_red)
        if isinstance(item, dict):
            total += sum_of_dict(item, ignore_red=ignore_red)
        if isinstance(item, int):
            total += item
    return total

def sum_of_dict(d, ignore_red=False):
    total = 0
    if ignore_red:
        if 'red' in d.values():
            return 0
    for _, item in d.iteritems():
        if isinstance(item, list):
            total += sum_of_list(item, ignore_red=ignore_red)
        if isinstance(item, dict):
            total += sum_of_dict(item, ignore_red=ignore_red)
        if isinstance(item, int):
            total += item
    return total

total = 0
for item in data:
    if isinstance(item, list):
        total += sum_of_list(item)
    if isinstance(item, dict):
        total += sum_of_dict(item)
    if isinstance(item, int):
        total += item

print(total)


total = 0
for item in data:
    if isinstance(item, list):
        total += sum_of_list(item, ignore_red=True)
    if isinstance(item, dict):
        total += sum_of_dict(item, ignore_red=True)
    if isinstance(item, int):
        total += item

print(total)
