def clean_data(line):
    line = line.strip()
    return line[0], int(line[1:])


with open('in') as f:
    data = list(map(clean_data, f.readlines()))


def part_one():
    x, y = 0, 0
    rot = 0  # east = 0, north = 1, west = 2, south = 3

    for action, value in data:
        if action == 'N':
            y += value
        elif action == 'S':
            y -= value
        elif action == 'E':
            x += value
        elif action == 'W':
            x -= value
        elif action == 'L':
            rot = (rot + value // 90) % 4
        elif action == 'R':
            rot = (rot - value // 90) % 4
        elif action == 'F':
            if rot == 0:
                x += value
            elif rot == 1:
                y += value
            elif rot == 2:
                x -= value
            elif rot == 3:
                y -= value

    return abs(x) + abs(y)


def part_two():
    x, y = 0, 0
    x_w, y_w = 10, 1

    for action, value in data:
        if action == 'N':
            y_w += value
        elif action == 'S':
            y_w -= value
        elif action == 'E':
            x_w += value
        elif action == 'W':
            x_w -= value
        elif action == 'L':
            for _ in range(value // 90):
                x_w, y_w = -y_w, x_w
        elif action == 'R':
            for _ in range(value // 90):
                x_w, y_w = y_w, -x_w
        elif action == 'F':
            x += x_w * value
            y += y_w * value

    return abs(x) + abs(y)


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
