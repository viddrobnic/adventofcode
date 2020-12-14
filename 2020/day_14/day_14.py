from collections import defaultdict


def clean_line(line):
    parts = line.strip().split(' = ')
    if parts[0] == 'mask':
        return parts

    mem_parts = parts[0].split('[')
    return [mem_parts[0], int(mem_parts[1][:-1]), int(parts[1])]


with open('in') as f:
    data = list(map(clean_line, f.readlines()))


def part_one():
    current_mask = 'X' * 36
    memory = defaultdict(int)

    for line in data:
        if line[0] == 'mask':
            current_mask = line[1]
        else:
            n = line[2]
            n |= int(current_mask.replace('X', '0'), 2)
            n &= int(current_mask.replace('X', '1'), 2)

            memory[line[1]] = n

    s = 0
    for k in memory.keys():
        s += memory[k]
    return s


def replace_x(val):
    if 'X' not in val:
        return [val]

    vals = []
    for i in range(36):
        if val[i] == 'X':
            val[i] = '1'
            vals += replace_x(val[:])

            val[i] = '0'
            vals += replace_x(val[:])

            break

    return vals


def part_two():
    current_mask = 'X' * 36
    memory = defaultdict(int)

    for line in data:
        if line[0] == 'mask':
            current_mask = line[1]
        else:
            n = list(bin(line[1])[2:])
            n = (['0'] * (36 - len(n))) + n
            for i in range(36):
                if current_mask[i] in ('1', 'X'):
                    n[i] = current_mask[i]

            values = map(lambda x: ''.join(x), replace_x(n))
            for v in values:
                memory[v] = line[2]

    s = 0
    for k in memory.keys():
        s += memory[k]

    return s


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
