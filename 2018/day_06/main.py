import sys
from collections import defaultdict


# Auxilery function
def distance(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)


# Read input
coordinates = []
for i in sys.stdin:
    x, y = map(int, i.strip().split(','))
    coordinates.append((x, y))


# Area for part 1
area = defaultdict(int)
on_edge = defaultdict(bool)

# Area for part 2
area2 = 0

# Square on which we focus our brute force
MIN, MAX = -100, 500
MAX_DISTANCE = 10000

# We brute force through it
for y in range(MIN, MAX):
    for x in range(MIN, MAX):
        min_d = None
        min_x, min_y = None, None
        same = False
        distance_sum = 0

        # Find coordinate with minimum distance
        for coord in coordinates:
            d = distance(x, y, coord[0], coord[1])
            distance_sum += d

            if min_d is None:
                min_d = d
                min_x, min_y = coord
            elif d < min_d:
                min_d = d
                min_x, min_y = coord
                same = False
            elif d == min_d:
                same = True

        # Part 2 calculations
        if distance_sum < MAX_DISTANCE:
            area2 += 1

        # If squares are the same do nothing
        if same:
            continue
        else:
            area[(min_x, min_y)] += 1

        # Remeber if this coordinate was on the edge
        if x == MIN or x == MAX - 1 or y == MIN or y == MAX - 1:
            on_edge[(min_x, min_y)] = True


# Find max area for part 1
max_area = 0
for c in area.keys():
    # If on edge, assume that the area is infinite
    if on_edge[c]:
        continue

    if area[c] > max_area:
        max_area = area[c]

print(f'Part 1: {max_area}')
print(f'Part 2: {area2}')


