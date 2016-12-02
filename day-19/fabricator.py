replacements = {}
molecule = None

with open('input.txt', 'r') as f:
    replacements_done = False
    for line in f.readlines():
        stripped = line.strip()
        if replacements_done:
            molecule = stripped
            break
        if stripped:
            frm, to = stripped.split(' => ')
            if frm in replacements:
                replacements[frm].append(to)
            else:
                replacements[frm] = [to]
        else:
            replacements_done = True

# part 2
start = 'e'
current = start
target = molecule

steps = 0
while current != target
    steps += 1
    current = make_replacement...
    
