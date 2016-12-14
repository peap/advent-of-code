lights = {}
for x in range(1000):
    for y in range(1000):
        coord = (x, y)
        lights[coord] = (False, 0)

def turn_state(is_on, level):
    if on_off == 'on':
        return (True, level + 1)
    if on_off == 'off':
        return (False, max([0, level - 1]))

def toggle_state(is_on, level):
    return (not is_on, level + 2)

with open('input.txt', 'r') as f:
    for line in f.readlines():
        instruction = line.strip()
        tokens = instruction.split()
        if tokens[0] == 'turn':
            _, on_off, start, _, end = tokens
            state_func = turn_state
        if tokens[0] == 'toggle':
            _, start, _, end = tokens
            state_func = toggle_state
 
        x_min, y_min = start.split(',')
        x_max, y_max = end.split(',')

        for x in range(int(x_min), int(x_max) + 1):
            for y in range(int(y_min), int(y_max) + 1):
                coord = (x, y)
                is_on, level = lights[coord]
                lights[coord] = state_func(is_on, level)

num_on = len([x for x in lights.values() if x[0]])

total_brightness = sum([x[1] for x in lights.values()])

print(num_on)
print(total_brightness)
 
