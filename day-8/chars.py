with open('input.txt', 'r') as f:
    lines = f.readlines()

diff = 0

for line in lines:
    string = line.strip()
    diff += len(string)
    # eeeeeeeeeeeeeee, eval()...
    diff -= len(eval(string))

print(diff)

diff = 0

for line in lines:
    string = line.strip()
    diff += len(repr(string).replace('"', r'\"'))
    # eeeeeeeeeeeeeee, eval()...
    diff -= len(string)

print(diff)
