floor = 0

with open('input.txt', 'r') as f:
    instructions = f.read()

step = 1
entered_basement = False

for instr in instructions:
    if instr == '(':
        floor += 1
    if instr == ')':
        floor -= 1
    if floor < 0 and not entered_basement:
        entered_basement = True
        print(step)
    step += 1

print(floor)
