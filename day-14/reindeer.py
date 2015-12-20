from pprint import pprint
import re

regex = re.compile(
    '(\w+) can fly (\d+) km/s for (\d+) seconds, '
    'but then must rest for (\d+) seconds.'
)

deer = {}

with open('input.txt', 'r') as f:
    for line in f.readlines():
        match = regex.match(line.strip())
        name, speed, duration, rest = match.groups()
        deer[name] = {
            'speed': int(speed),
            'duration': int(duration),
            'rest': int(rest),
        }

def get_distance(data, seconds):
    period = data['duration'] + data['rest']
    dist_per_period = data['speed'] * data['duration']
    n_periods = seconds // period
    n_extra = seconds % period

    distance = dist_per_period * n_periods
    if n_extra > data['duration']:
        distance += dist_per_period
    else:
        distance += data['speed'] * n_extra

    return distance

race_time = 2503

distances = []
for name, data in deer.iteritems():
    distance = get_distance(data, race_time)
    distances.append((name, distance))

for n, d in sorted(distances, key=lambda x: -1 * x[1]):
    print('{0}: {1} km'.format(n, d))

points = {name: 0 for name in deer}
for s in range(1, race_time + 1):
    dists = {}
    for name, data in deer.iteritems():
        dists[name] = get_distance(data, s)
    max_dist = max([val for _, val in dists.iteritems()])
    for name, dist in dists.iteritems():
        if dist == max_dist:
            points[name] += 1

pprint(points)
