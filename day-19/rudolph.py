import re

element_re = re.compile(r'([A-Z]([a-z])?)')

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

new_molecules = set()
unreplaceable = set()
for i in range(len(molecule)):
    element = molecule[i:i+2]
    if element in replacements:
        start, end = i, i + 2
    else:
        start, end = i, i + 1
        element = molecule[i]
    element_replacements = replacements.get(element, [])
    if element_replacements:
        for new_element in element_replacements:
            new_molecule = molecule[:start] + new_element + molecule[end:]
            new_molecules.add(new_molecule)
    else:
        unreplaceable.add(element)

print('# new molecules: {0}'.format(len(new_molecules)))


# part 2

#def replace(mol, index)
#
#done = False
#while not done:
#    steps = 0
#    m = 'e'
#    
