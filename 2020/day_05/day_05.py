with open('in') as f:
    data = list(map(lambda x: x.strip(), f.readlines()))


def code_to_int(code):
    low, high = 0, 2 ** len(code) - 1
    for c in code:
        if c in 'FL':
            high = low + (high - low) // 2
        else:
            low = high - (high - low) // 2

    assert low == high
    return low


def part_one():
    max_id = 0
    for seat in data:
        row = code_to_int(seat[:7])
        column = code_to_int(seat[-3:])

        seat_id = row * 8 + column
        max_id = max(max_id, seat_id)

    return max_id


def part_two():
    existing_ids = set()
    for seat in data:
        row = code_to_int(seat[:7])
        column = code_to_int(seat[-3:])

        seat_id = row * 8 + column
        existing_ids.add(seat_id)

    for seat_id in range(0, 128 * 8):
        if seat_id not in existing_ids and seat_id - 1 in existing_ids and seat_id + 1 in existing_ids:
            return seat_id


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
