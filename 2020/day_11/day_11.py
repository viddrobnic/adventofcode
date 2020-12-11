from copy import deepcopy

with open('in') as f:
    data = list(map(lambda x: list(x.strip()), f.readlines()))
    h = len(data)
    w = len(data[0])


def count_occupied_1(i, j, seats):
    adj_occupied = 0

    for dx, dy in ((-1, -1), (-1, 0), (0, -1), (1, 1), (0, 1), (1, 0), (-1, 1), (1, -1)):
        y, x = i + dy, j + dx
        if x < 0 or x >= w or y < 0 or y >= h:
            continue

        elif seats[y][x] == '#':
            adj_occupied += 1

    return adj_occupied


def count_occupied_2(i, j, seats):
    adj_occupied = 0

    for dx, dy in ((-1, -1), (-1, 0), (0, -1), (1, 1), (0, 1), (1, 0), (-1, 1), (1, -1)):
        y, x = i + dy, j + dx
        while x >= 0 and x < w and y >= 0 and y < h:
            if seats[y][x] == '#':
                adj_occupied += 1

            if seats[y][x] != '.':
                break

            y, x = y + dy, x + dx

    return adj_occupied


def solver(part):
    seats = deepcopy(data)

    if part == 1:
        min_occ = 4
    else:
        min_occ = 5

    while True:
        changes = []
        for i in range(h):
            for j in range(w):
                if part == 1:
                    adj_occupied = count_occupied_1(i, j, seats)
                else:
                    adj_occupied = count_occupied_2(i, j, seats)

                if seats[i][j] == 'L':
                    if adj_occupied == 0:
                        changes.append((i, j))
                elif seats[i][j] == '#':
                    if adj_occupied >= min_occ:
                        changes.append((i, j))

        if len(changes) == 0:
            break

        for i, j in changes:
            if seats[i][j] == 'L':
                seats[i][j] = '#'
            elif seats[i][j] == '#':
                seats[i][j] = 'L'

    return sum([sum(map(lambda x: x == '#', seats[i])) for i in range(h)])


if __name__ == '__main__':
    print(f'Part One: {solver(1)}')
    print(f'Part Two: {solver(2)}')
