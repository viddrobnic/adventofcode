def clean_data(line):
    parts = line.split()
    p1 = parts[0]
    minimum, maximum = map(int, p1.split('-'))
    return minimum, maximum, parts[1][0], parts[2]


with open('in') as f:
    data = list(map(clean_data, f.readlines()))


def part_one():
    nr_valid = 0
    for (m, M, c, p) in data:
        count = p.count(c)
        if count >= m and count <= M:
            nr_valid += 1

    return nr_valid


def part_two():
    nr_valid = 0
    for (i, j, c, p) in data:
        count = (p[i - 1] == c) + (p[j - 1] == c)
        if count == 1:
            nr_valid += 1

    return nr_valid


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
