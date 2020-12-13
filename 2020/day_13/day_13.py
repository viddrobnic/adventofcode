import math

with open('in') as f:
    earliest_departure = int(f.readline())
    buses = list(map(lambda b: (b[0], int(b[1])), [b for b in enumerate(f.readline().split(',')) if b[1] != 'x']))


def part_one():
    departure = None
    bus = None

    for b in buses:
        t = math.ceil(earliest_departure / b[1]) * b[1]
        if departure is None or t < departure:
            departure = t
            bus = b[1]

    return bus * (departure - earliest_departure)


def part_two():
    N = 1
    for _, b in buses:
        N *= b

    t = 0
    for i, b in buses:
        Ni = N // b
        Mi = pow(Ni, -1, b)
        t += -i * Mi * Ni

    return t % N


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
