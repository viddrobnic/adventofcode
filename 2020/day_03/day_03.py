with open('in') as f:
    data = list(map(lambda x: x.strip(), f.readlines()))
h, w = len(data), len(data[0])


def part_one():
    nr_trees = 0
    x, y = 0, 0
    for i in range(h - 1):
        x = (x + 3) % w
        y += 1
        if data[y][x] == '#':
            nr_trees += 1

    return nr_trees


def part_two():
    slopes = ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
    res = 1
    for dx, dy in slopes:
        nr_trees = 0
        x, y = 0, 0
        while y + dy < h:
            x = (x + dx) % w
            y += dy
            if data[y][x] == '#':
                nr_trees += 1

        res *= nr_trees

    return res


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
