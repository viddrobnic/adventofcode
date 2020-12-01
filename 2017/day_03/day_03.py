from collections import defaultdict

number = int(input())

values = defaultdict(int)
values[(0, 0)] = 1

x, y = (0, 0)
direction = 0
directions = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]
data = 1
second_result = 0
length = 1
step = 0
rotations = 0

first, second = False, False
while not (first and second):
    # Add a step
    d = directions[direction]
    x += d[0]
    y += d[1]

    step += 1
    # If number of steps equals length of the current trajectory, then rotate
    if step >= length:
        direction = (direction + 1) % 4
        step = 0
        rotations += 1
        # Every two rotations length of the side is 1 longer
        if rotations == 2:
            length += 1
            rotations = 0

    if not first:
        data += 1
    if data == number:
        first = True

    # Sum of all adjecant squares
    adjecant_sum = 0
    for d in directions:
        x_1 = x + d[0]
        y_1 = y + d[1]
        adjecant_sum += values[(x_1, y_1)]
    values[(x, y)] = adjecant_sum

    if not second and adjecant_sum > number:
        second_result = adjecant_sum
        second = True

print('Part One: {}\nPart Two: {}'.format(abs(x) + abs(y), second_result))
