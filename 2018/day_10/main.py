import sys


# Read input
positions = []
velocities = []
for line in sys.stdin:
    line = line.strip().split('<')
    pos = line[1].split('>')[0]
    pos = pos.split(',')

    x = int(pos[0].strip())
    y = int(pos[1].strip())

    vel = line[2]
    vel = vel.replace('>', '')
    vel = vel.split(',')

    vel_x = int(vel[0].strip())
    vel_y = int(vel[1].strip())

    positions.append((x, y))
    velocities.append((vel_x, vel_y))


# Function to calculate larger side of the rectangle
# in which there are all the points
def get_extremes(positions):
    min_x, max_x, min_y, max_y = None, None, None, None
    for pos in positions:
        x, y = pos
        if min_x is None or x < min_x:
            min_x = x

        if max_x is None or x > max_x:
            max_x = x

        if min_y is None or y < min_y:
            min_y = y

        if max_y is None or y > max_y:
            max_y = y

    return min_x, max_x, min_y, max_y


# Function that prints all the points with nice ascii art
def print_board(positions):
    min_x, max_x, min_y, max_y = get_extremes(positions)

    poses = set(positions)
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if (x, y) in poses:
                print('#', end='')
            else:
                print(' ', end='')
        print()


# Preapre count for part 2
count = 0
# While max side of the rectangle is getting smaller, the points are
# getting more together. We are looking for the minimum of this distance
prev_max_diff = None
prev_poses = None

while True:
    # Count time for part 2
    count += 1

    # Move points
    for i in range(len(positions)):
        vel_x, vel_y = velocities[i]
        x, y = positions[i]
        positions[i] = (x + vel_x, y + vel_y)

    # Get extremes
    min_x, max_x, min_y, max_y = get_extremes(positions)
    d1 = abs(max_x - min_x)
    d2 = abs(max_y - min_y)
    max_d = max(d1, d2)

    if prev_max_diff is None:
        prev_max_diff = max_d
        prev_poses = positions[:]
        continue

    # We found our minimum, so we print it
    if max_d > prev_max_diff:
        print('Part 1:')
        print_board(prev_poses)
        print()

        print(f'Part 2: {count - 1}')
        break
    else:
        prev_poses = positions[:]
        prev_max_diff = max_d

