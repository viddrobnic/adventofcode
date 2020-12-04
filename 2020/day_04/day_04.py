with open('in') as f:
    data = []
    current = dict()
    for line in f:
        line = line.strip()
        if line == '':
            data.append(current)
            current = dict()
            continue

        parts = line.split()
        for part in parts:
            key, value = part.split(':')
            current[key] = value

    data.append(current)


def part_one():
    required_keys = ('byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid')

    nr_valid = 0
    for p in data:
        valid = True
        for k in required_keys:
            if k not in p:
                valid = False
                break

        if valid:
            nr_valid += 1

    return nr_valid


def in_range(n, m, M):
    return n >= m and n <= M


def part_two():
    nr_valid = 0
    for passport in data:
        # Check `byr`
        if 'byr' not in passport:
            continue
        byr = int(passport['byr'])
        if byr < 1920 or byr > 2002:
            continue

        # Check `iyr`
        if 'iyr' not in passport:
            continue
        iyr = int(passport['iyr'])
        if not in_range(iyr, 2010, 2020):
            continue

        # Check eyr
        if 'eyr' not in passport:
            continue
        eyr = int(passport['eyr'])
        if not in_range(eyr, 2020, 2030):
            continue

        # Check height
        if 'hgt' not in passport:
            continue
        try:
            hgt = int(passport['hgt'][:-2])
        except:
            continue
        enote = passport['hgt'][-2:]
        if enote == 'cm':
            if not in_range(hgt, 150, 193):
                continue
        elif enote == 'in':
            if not in_range(hgt, 59, 76):
                continue
        else:
            continue

        # Check hcl
        if 'hcl' not in passport:
            continue
        hcl = passport['hcl']
        if hcl[0] != '#':
            continue
        valid = True
        for c in hcl[1:]:
            if c not in '0123456789abcdef':
                valid = False
                break
        if not valid:
            continue

        # Check ecl
        if 'ecl' not in passport:
            continue
        if passport['ecl'] not in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'):
            continue

        # Check pid
        if 'pid' not in passport:
            continue
        if len(passport['pid']) != 9:
            continue

        nr_valid += 1

    return nr_valid


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
