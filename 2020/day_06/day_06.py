with open('in') as f:
    data = list(map(lambda x: x.strip(), f.readlines()))


def part_one():
    count = 0
    group = set()

    for line in data:
        if line == '':
            count += len(group)
            group = set()
            continue

        group.update(line)

    count += len(group)
    return count


def part_two():
    count = 0
    group = set()
    empty = True

    for line in data:
        if line == '':
            count += len(group)
            group = set()
            empty = True
            continue

        if empty:
            group.update(line)
            empty = False
        else:
            group.intersection_update(line)

    count += len(group)
    return count


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
