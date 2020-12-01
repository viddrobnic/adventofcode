steps = input().split(',')
moves = {
    'n': (-1, 0, 1),
    'ne': (-1, 1, 0),
    'se': (0, 1, -1),
    's': (1, 0, -1),
    'sw': (1, -1, 0),
    'nw': (0, -1, 1)
}

def distance(x, y, z):  # Caluclate distance
    return (abs(x) + abs(y) + abs(z)) // 2

distances = []
x, y, z = 0, 0, 0
for i in steps:
    x += moves[i][0]
    y += moves[i][1]
    z += moves[i][2]
    distances.append(distance(x, y, z))
print('Part One: {}\nPart Two: {}'.format(distance(x, y, z), max(distances)))
