with open('in') as f:
    data = list(map(lambda l: l.strip(), f.readlines()))


def identify_tile(line):
    x, y = 0, 0

    i = 0
    while i < len(line):
        c = line[i]

        if c == 'e':
            x += 2
        elif c == 'w':
            x -= 2
        elif c == 's':
            y -= 2

            if line[i + 1] == 'w':
                x -= 1
            elif line[i + 1] == 'e':
                x += 1

            i += 1

        elif c == 'n':
            y += 2

            if line[i + 1] == 'w':
                x -= 1
            elif line[i + 1] == 'e':
                x += 1

            i += 1

        i += 1

    return x, y


def part_one():
    flipped = dict()

    for line in data:
        x, y = identify_tile(line)
        if (x, y) in flipped:
            flipped[(x, y)] = not flipped[(x, y)]
        else:
            flipped[(x, y)] = True

    return flipped


def adjacent_black(tiles, x, y):
    adjacent = [(2, 0), (1, 2), (-1, 2), (1, -2), (-1, -2), (-2, 0)]

    res = 0
    for dx, dy in adjacent:
        if (x + dx, y + dy) not in tiles:
            continue

        if tiles[(x + dx, y + dy)]:
            res += 1

    return res


def part_two():
    adjacent = [(2, 0), (1, 2), (-1, 2), (1, -2), (-1, -2), (-2, 0)]

    tiles = part_one()

    for i in range(100):
        to_change = set()

        for x, y in tiles.keys():
            if not tiles[(x, y)]:
                continue

            nr_black = adjacent_black(tiles, x, y)
            if nr_black == 0 or nr_black > 2:
                to_change.add((x, y))

            for dx, dy in adjacent:
                if (x + dx, y + dy) in tiles and tiles[(x + dx, y + dy)]:
                    continue

                if adjacent_black(tiles, x + dx, y + dy) == 2:
                    to_change.add((x + dx, y + dy))

        for x, y in to_change:
            if (x, y) in tiles:
                tiles[(x, y)] = not tiles[(x, y)]
            else:
                tiles[(x, y)] = True

    return sum(tiles.values())


if __name__ == '__main__':
    print(f'Part One: {sum(part_one().values())}')
    print(f'Part Two: {part_two()}')
