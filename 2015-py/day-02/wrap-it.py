total_area = 0
total_ribbon = 0

with open('input.txt', 'r') as f:
    for line in f.readlines():
        dims = [int(s) for s in line.strip().split('x')]

        l, w, h = dims
        main_area = 2 * (l * w + l * h + w * h)

        volume = l * w * h

        max_dim = max(dims)

        max_dim_index = dims.index(max_dim)
        dims.pop(max_dim_index)
        extra_area = dims[0] * dims[1]

        ribbon_length = 2 * sum(dims) + volume

        total_area += main_area + extra_area
        total_ribbon += ribbon_length

print(total_area)
print(total_ribbon)
