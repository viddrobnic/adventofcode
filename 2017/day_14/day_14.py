from collections import deque

key = input()
moves = [(-1, 0), (0, -1), (1, 0), (0, 1)]  # Posible moves (used for part two)

def knot_hash(h):  # Copied from day 10
    size = 256
    data = [i for i in range(size)]
    skip_size = 0
    current_position = 0

    lengths = list(map(ord, h))
    lengths += [17, 31, 73, 47, 23]

    for _ in range(64):
        for i in lengths:
            buff = []
            for j in range(current_position, current_position + i):
                buff.append(data[j % size])

            buff = buff[::-1]

            for j in range(len(buff)):
                data[(j + current_position) % size] = buff[j]

            current_position += i + skip_size
            current_position %= size
            skip_size += 1

    d_hash = ''
    for i in range(16):
        block = 0
        for j in range(i * 16, i* 16 + 16):
            block ^= data[j]
        d_hash += hex(block)[2:].zfill(2)
    return d_hash

used = 0  # Number of used bits
grid = []  # 2d array representing the grid

for i in range(128):
    hex_hash = knot_hash('{}-{}'.format(key, i))  # For each line create a hex
    line = ''.join(['{0:04b}'.format(int(i, base=16)) for i in hex_hash])  # Convert hex to bits
    grid.append(list(line))  # Populate the grid
    used += line.count('1')  # Count used bits

seen = set()  # Seen used in BFS
groups = 0  # Number of groups in grids
for y in range(128):
    for x in range(128):
        if (x, y) in seen:  # If coordinate already visited ignore it
            continue
        if grid[y][x] == '0':  # If coordinate is unused ignore it
            continue

        groups += 1  # Beginning of new group, since coordinate hasn't yet been visited
        que = deque()
        que.append((x, y))
        seen.add((x, y))

        while len(que) > 0:  # BFS through all coordinates touching the current one
            x_1, y_1 = que.popleft()
            for move in moves:
                x_2 = x_1 + move[0]  # Calculate moved coordinate
                y_2 = y_1 + move[1]
                if x_2 < 0 or x_2 > 127 or y_2 < 0 or y_2 > 127:  # Check if out of bounds
                    continue
                if grid[y_2][x_2] == '0':  # Check if unused
                    continue
                if (x_2, y_2) in seen:  # Check if already visited
                    continue
                seen.add((x_2, y_2))
                que.append((x_2, y_2))

print('Part One: {}\nPart Two: {}'.format(used, groups))
