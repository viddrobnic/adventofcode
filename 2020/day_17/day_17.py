import copy

active_cubes_3 = dict()
active_cubes_4 = dict()
with open('in') as f:
    for y, l in enumerate(f.readlines()):
        for x, c in enumerate(l.strip()):
            if c == '#':
                active_cubes_3[(x, y, 0)] = True
                active_cubes_4[(x, y, 0, 0)] = True


def neighbours_3(x, y, z):
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            for dz in (-1, 0, 1):
                if dx == 0 and dy == 0 and dz == 0:
                    continue
                else:
                    yield x + dx, y + dy, z + dz


def neighbours_4(x, y, z, w):
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            for dz in (-1, 0, 1):
                for dw in (-1, 0, 1):
                    if dx == 0 and dy == 0 and dz == 0 and dw == 0:
                        continue
                    else:
                        yield x + dx, y + dy, z + dz, w + dw


def active_neighbors(pos, cubes, neigh_func):
    act_neigh = 0
    for n in neigh_func(*pos):
        if n in cubes and cubes[n]:
            act_neigh += 1

    return act_neigh


def cycle(cubes, neigh_func):
    to_change = set()
    seen = set()

    for cube in cubes.keys():
        if not cubes[cube]:
            continue

        act_neigh = active_neighbors(cube, cubes, neigh_func)

        if not (act_neigh == 2 or act_neigh == 3):
            to_change.add(cube)

        for n in neigh_func(*cube):
            if n in seen:
                continue
            seen.add(n)

            if n not in cubes or not cubes[n]:
                if active_neighbors(n, cubes, neigh_func) == 3:
                    to_change.add(n)

    return to_change


def solver(initial_cubes, neigh_func):
    cubes = copy.deepcopy(initial_cubes)

    for i in range(6):
        to_change = cycle(cubes, neigh_func)
        for c in to_change:
            if c in cubes:
                val = cubes[c]
            else:
                val = False
            cubes[c] = not val

    return sum(cubes.values())


def part_one():
    return solver(active_cubes_3, neighbours_3)


def part_two():
    return solver(active_cubes_4, neighbours_4)


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
