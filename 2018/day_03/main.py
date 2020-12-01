import sys
from collections import defaultdict


claims = []
cloth = defaultdict(int)

# Read the data
for i in sys.stdin:
    if i.strip() == '':
        continue
    a = i.strip().split()
    ID = a[0][1:]
    left, top = map(int, a[2][:-1].split(','))
    width, height = map(int, a[3].split('x'))
    claims.append((ID, left, top, width, height))

    for y in range(top, top + height):
        for x in range(left, left + width):
            cloth[(x, y)] += 1

# Part 1
count = 0
for val in cloth.values():
    if val >= 2:
        count += 1

print(f'Part 1: {count}')

# Part 2
non_overlaping_id = None
for claim in claims:
    ID, left, top, width, height = claim
    overlaping = True
    for y in range(top, top + height):
        for x in range(left, left + width):
            if cloth[(x, y)] != 1:
                overlaping = False
                break
        if not overlaping:
            break
    if overlaping:
        non_overlaping_id = ID
        break

print(f'Part 2: {non_overlaping_id}')

