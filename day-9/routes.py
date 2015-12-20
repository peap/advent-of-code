from pprint import pprint
import re

instruction_re = re.compile('([\w]+) to ([\w]+) = ([0-9]+)')

city_graph = {}

with open('input.txt', 'r') as f:
    for line in f.readlines():
        match = instruction_re.match(line.strip())
        start, stop, distance = match.groups()
        if start in city_graph:
            city_graph[start].append((stop, int(distance)))
        else:
            city_graph[start] = [(stop, int(distance))]
        if stop in city_graph:
            city_graph[stop].append((start, int(distance)))
        else:
            city_graph[stop] = [(start, int(distance))]

cities = city_graph.keys()

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

for start_city in cities:
    for end_city in cities:
        if start_city == end_city:
            continue
        paths = find_paths(city_graph, start_city, end_city)
        for path in paths:
            if len(path) == len(cities):
                distance = 0
                for i in range(len(path) - 1):
                    s, e = path[i], path[i+1]
                    for dest, dist in city_graph[s]:
                        if dest == e:
                            distance += dist
                good_paths.append((path, distance))


sorted_paths = sorted(good_paths, key=lambda p: p[1])

print('shortest: {0}'.format(sorted_paths[0][1]))
print('longest:  {0}'.format(sorted_paths[-1][1]))
