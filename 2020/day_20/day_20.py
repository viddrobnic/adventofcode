from collections import deque

tiles = dict()

with open('in') as f:
    tile = []
    tile_id = None

    for line in f.readlines():
        line = line.strip()
        if line == '':
            tiles[tile_id] = tile
            tile = []
        elif line.startswith('Tile'):
            tile_id = int(line.split()[1][:-1])
        else:
            tile.append(line)

    if len(tile) > 0:
        tiles[tile_id] = tile


def rotate_left(tile):
    new_tile = [''] * len(tile)
    for line in tile:
        for j in range(len(line)):
            new_tile[-j - 1] += line[j]

    return new_tile


def flip_vertical(tile):
    return tile[::-1]


def get_left_edge(tile):
    left_edge = ''
    for j in range(len(tile)):
        left_edge += tile[j][0]

    return left_edge


def get_right_edge(tile):
    right_edge = ''
    for j in range(len(tile)):
        right_edge += tile[j][-1]

    return right_edge


def find_left_edge(edge, tile_id):
    for tile_id2 in tiles.keys():
        if tile_id2 == tile_id:
            continue

        tile = tiles[tile_id2]
        for i in range(4):
            if get_left_edge(tile) == edge:
                return tile, tile_id2

            flipped = flip_vertical(tile)
            if get_left_edge(flipped) == edge:
                return flipped, tile_id2

            tile = rotate_left(tile)

    return None, None


def find_bottom_edge(edge, tile_id):
    tile, tile_id = find_left_edge(edge, tile_id)
    if tile is None:
        return None, None

    return rotate_left(tile), tile_id


def find_right_edge(edge, tile_id):
    tile, tile_id = find_left_edge(edge, tile_id)
    if tile is None:
        return None, None

    return flip_vertical(rotate_left(rotate_left(tile))), tile_id


def find_top_edge(edge, tile_id):
    tile, tile_id = find_left_edge(edge, tile_id)
    if tile is None:
        return None, None

    return flip_vertical(rotate_left(tile)), tile_id


def join():
    moves = [(0, 1), (-1, 0), (0, -1), (1, 0)]

    image = dict()
    q = deque()

    starting_id = list(tiles.keys())[0]
    starting_tile = tiles[starting_id]
    image[(0, 0)] = (starting_id, starting_tile)
    q.append((starting_tile[0], starting_id, 0, 0, 0))
    q.append((starting_tile[-1], starting_id, 0, 0, 2))
    q.append((get_left_edge(starting_tile), starting_id, 0, 0, 1))
    q.append((get_right_edge(starting_tile), starting_id, 0, 0, 3))

    while q:
        edge, tile_id, x, y, orientation = q.popleft()
        dx, dy = moves[orientation]
        if (x + dx, y + dy) in image:
            continue

        tile, tile_id2 = None, None
        if orientation == 0:
            tile, tile_id2 = find_bottom_edge(edge, tile_id)
            if tile is not None:
                assert tile[-1] == edge
        elif orientation == 1:
            tile, tile_id2 = find_right_edge(edge, tile_id)
            if tile is not None:
                assert get_right_edge(tile) == edge
        elif orientation == 2:
            tile, tile_id2 = find_top_edge(edge, tile_id)
            if tile is not None:
                assert tile[0] == edge
        elif orientation == 3:
            tile, tile_id2 = find_left_edge(edge, tile_id)
            if tile is not None:
                assert get_left_edge(tile) == edge

        if tile is None:
            continue

        image[(x + dx, y + dy)] = (tile_id2, tile)

        q.append((tile[0], tile_id2, x + dx, y + dy, 0))
        q.append((tile[-1], tile_id2, x + dx, y + dy, 2))
        q.append((get_left_edge(tile), tile_id2, x + dx, y + dy, 1))
        q.append((get_right_edge(tile), tile_id2, x + dx, y + dy, 3))

    return image


def nr_monsters(img):
    to_match = [(0, 0), (1, 1), (4, 1), (5, 0), (6, 0), (7, 1), (10, 1), (11, 0), (12, 0), (13, 1), (16, 1), (17, 0),
                (18, 0), (19, 0), (18, -1)]

    res = 0

    for y in range(len(img)):
        for x in range(len(img[0])):
            contains = True
            for dx, dy in to_match:
                x1, y1 = x + dx, y + dy
                if x1 < 0 or x1 >= len(img[0]) or y1 < 0 or y1 >= len(img):
                    contains = False
                    break

                if img[y1][x1] != '#':
                    contains = False
                    break

            if contains:
                res += 1
    return res


def part_one(image):
    min_x = min(map(lambda x: x[0], image.keys()))
    max_x = max(map(lambda x: x[0], image.keys()))
    min_y = min(map(lambda x: x[1], image.keys()))
    max_y = max(map(lambda x: x[1], image.keys()))

    return image[(min_x, min_y)][0] * image[(min_x, max_y)][0] * image[(max_x, min_y)][0] * image[(max_x, max_y)][0]


def part_two(image):
    min_x = min(map(lambda x: x[0], image.keys()))
    max_x = max(map(lambda x: x[0], image.keys()))
    min_y = min(map(lambda x: x[1], image.keys()))
    max_y = max(map(lambda x: x[1], image.keys()))

    h = len(image[(min_x, min_y)][1]) - 2

    img = []
    for y in range(max_y, min_y - 1, -1):
        img += [''] * h

        for x in range(min_x, max_x + 1):
            for i in range(1, h + 1):
                img[len(img) - h + i - 1] += image[(x, y)][1][i][1:-1]

    for i in range(4):
        nr = nr_monsters(img)
        if nr > 0:
            return sum(map(lambda l: l.count('#'), img)) - 15 * nr

        flipped = flip_vertical(img)
        nr = nr_monsters(flipped)
        if nr > 0:
            return sum(map(lambda l: l.count('#'), flipped)) - 15 * nr

        img = rotate_left(img)


if __name__ == '__main__':
    image = join()
    print(f'Part One: {part_one(image)}')
    print(f'Part Two: {part_two(image)}')
