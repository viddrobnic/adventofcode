with open('in') as f:
    data = list(map(lambda x: int(x.strip()), f.readlines()))


def part_one():
    for i in range(25, len(data)):
        valid = False
        for j in range(i - 25, i):
            found = False
            for k in range(i - 25, i):
                if data[i] == data[j] + data[k] and data[j] != data[k]:
                    valid = True
                    found = True
                    break

            if found:
                break

        if not valid:
            return data[i]


def part_two(n):
    for size in range(2, len(data) + 1):
        for i in range(size, len(data)):
            cset = data[i - size:i]
            if sum(cset) == n:
                return min(cset) + max(cset)


if __name__ == '__main__':
    n = part_one()
    print(f'Part One: {n}')
    print(f'Part Two: {part_two(n)}')
