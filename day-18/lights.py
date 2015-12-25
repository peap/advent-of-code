from copy import deepcopy
from itertools import combinations
from pprint import pprint

ON = '#'
OFF = '.'

N_STEPS = 100

initial_grid = []
with open('input.txt', 'r') as f:
    for row in f.readlines():
        grid_row = []
        for light in row.strip():
            grid_row.append(light)
        initial_grid.append(grid_row)

n_rows = len(initial_grid)
n_cols = len(initial_grid[0])

CORNERS = (
    (0,          0),
    (n_rows - 1, 0),
    (0,          n_cols - 1),
    (n_rows - 1, n_cols - 1),
)

def get_neighbors(coords):
    x, y = coords
    xvals = [x - 1, x, x + 1]
    yvals = [y - 1, y, y + 1]
    neighbors = []
    for px, py in [(x1, y1) for x1 in xvals for y1 in yvals]:
        if px < 0 or px >= n_rows:
            continue
        if py < 0 or py >= n_cols:
            continue
        if px == x and py == y:
            continue
        neighbors.append((px, py))
    return neighbors

def get_new_state(grid, coords, corners_on=False):
    if corners_on and (coords in CORNERS):
        return ON
    x, y = coords
    current_state = grid[x][y]
    neighbors = get_neighbors(coords)
    n_on = 0
    for nx, ny in neighbors:
        n_on += grid[nx][ny] == ON
    if current_state == ON:
        return ON if n_on in (2, 3) else OFF
    else:
        return ON if n_on == 3 else OFF

# do steps (part 1)
grid = initial_grid
for step in range(N_STEPS):
    new_grid = deepcopy(grid)
    for x in range(n_cols):
        for y in range(n_rows):
            new_grid[x][y] = get_new_state(grid, (x, y), corners_on=False)
    grid = new_grid

n_on = sum([
    sum([l == ON for l in row])
    for row in grid
])
print('after {0} steps, {1} lights are on'.format(N_STEPS, n_on))

# do steps (part 2)
grid = initial_grid
for x, y in CORNERS:
    grid[x][y] = ON
for step in range(N_STEPS):
    new_grid = deepcopy(grid)
    for x in range(n_cols):
        for y in range(n_rows):
            new_grid[x][y] = get_new_state(grid, (x, y), corners_on=True)
    grid = new_grid

n_on = sum([
    sum([l == ON for l in row])
    for row in grid
])
print('after {0} steps, {1} lights are on'.format(N_STEPS, n_on))
