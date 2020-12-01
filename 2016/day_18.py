previous_tiles = '.^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^...^^.^.^.......^..^^...^.^.^^..^^^^^...^.'


def generate_tile(left, center, right):
    if left == '^' and center == '^' and right == '.':
        return '^'
    if center == '^' and right == '^' and left == '.':
        return '^'
    if left == '^' and right == '.' and center == '.':
        return '^'
    if right == '^' and center == '.' and left == '.':
        return '^'
    return '.'


count_1 = previous_tiles.count('.')
count_2 = previous_tiles.count('.')

for rows in range(400000-1):
    current = ''
    for i in range(len(previous_tiles)):
        left = previous_tiles[i-1] if i > 0 else '.'
        center = previous_tiles[i]
        right = previous_tiles[i+1] if i < len(previous_tiles) - 1 else '.'
        current += generate_tile(left, center, right)


    count = current.count('.')
    if rows < 39:
        count_1 += count
    count_2 += count

    previous_tiles = current[:]

print('#1:', count_1)
print('#2:', count_2)
