data = []

with open('in') as f:
    for line in f.readlines():
        parts = line.strip().replace(')', '').split('(contains')
        data.append((set(parts[0].split()), set(map(lambda x: x.strip(), parts[1].split(',')))))


def part_one(candidates):
    res = 0
    for ings, _ in data:
        for ing in ings:
            valid = False
            for i in candidates.values():
                if ing in i:
                    valid = True
                    break
            if not valid:
                res += 1

    return res


def part_two(candidates):
    for _ in range(1000):
        for allergen in candidates.keys():
            if len(candidates[allergen]) == 1:
                for allergen2 in candidates.keys():
                    if allergen == allergen2:
                        continue

                    candidates[allergen2] -= candidates[allergen]

    ingredients = []
    for allergen in sorted(candidates.keys()):
        ingredients.append(candidates[allergen].pop())

    return ','.join(ingredients)


def main():
    allergens = set()
    ingredients = set()
    for ing, al in data:
        allergens |= al
        ingredients |= ing

    candidates = dict()
    for allergen in allergens:
        candidates[allergen] = ingredients
        for ing, al in data:
            if allergen in al:
                candidates[allergen] = candidates[allergen].intersection(ing)

    print(f'Part One: {part_one(candidates)}')
    print(f'Part Two: {part_two(candidates)}')


if __name__ == '__main__':
    main()
