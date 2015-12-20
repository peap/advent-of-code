import re

regex = re.compile(
    '(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).'
)

happies = []

with open('input2.txt', 'r') as f:
    lines = f.readlines()
    for line in lines:
        match = regex.match(line.strip())
        happies.append(match.groups())

happydict = {}
for item in happies:
    subject, gain_lose, amount, object = item
    factor = 1 if gain_lose == 'gain' else -1
    distance = factor * int(amount)
    if subject in happydict:
        happydict[subject].append((object, distance))
    else:
        happydict[subject] = [(object, distance)]

graph = {}
for person, others in happydict.iteritems():
    graph[person] = []
    for other, myhappy in others:
        for otherother, otherhappy in happydict[other]:
            if otherother == person:
                net_happy = otherhappy + myhappy
                break
        graph[person].append((other, net_happy))

def find_paths(graph, start, end, path=[]):
    path = path + [start]
    if start == end:
        return [path]
    if start not in graph:
        return []
    paths = []
    for node, distance in graph[start]:
        if node not in path:
            newpaths = find_paths(graph, node, end, path)
            for newpath in newpaths:
                paths.append(newpath)
    return paths

good_paths = []
people = graph.keys()
for start_person in people:
    for end_person in people:
        if start_person == end_person:
            continue
        paths = find_paths(graph, start_person, end_person)
        for path in paths:
            if len(path) == len(people):
                distance = 0
                for i in range(len(path) - 1):
                    s, e = path[i], path[i+1]
                    for dest, dist in graph[s]:
                        if dest == e:
                            distance += dist
                # finish path
                first, last = path[0], path[-1]
                for other, net in graph[first]:
                    if other == last:
                        distance += net
                        break
                good_paths.append((path, distance))


sorted_paths = sorted(good_paths, key=lambda p: p[1])

happiest_path, happy_total = sorted_paths[-1]

print(happy_total)

#print('shortest: {0}'.format(sorted_paths[0][1]))
#print('longest:  {0}'.format(sorted_paths[-1][1]))
#print('longest:  {0}'.format(', '.join(sorted_paths[-1][0])))
