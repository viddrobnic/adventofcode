def play(cups, start_cup, nr_moves):
    current_cup = start_cup
    max_cup = max(cups)

    for move in range(nr_moves):
        picked_cups = [None] * 3
        last_cup = current_cup
        for i in range(3):
            picked_cups[i] = cups[last_cup]
            last_cup = cups[last_cup]

        cups[current_cup] = cups[last_cup]

        destination_cup = current_cup - 1
        if destination_cup <= 0:
            destination_cup = max_cup
        while destination_cup in picked_cups:
            destination_cup -= 1
            if destination_cup <= 0:
                destination_cup = max_cup

        next_cup = cups[destination_cup]
        cups[destination_cup] = picked_cups[0]
        cups[picked_cups[2]] = next_cup

        current_cup = cups[current_cup]

    return cups


def part_one(cups, start_cup):
    cups = play(cups, start_cup, 100)

    res = ''
    current_cup = cups[1]
    while current_cup != 1:
        res += str(current_cup)
        current_cup = cups[current_cup]

    return res


def part_two(cups, start_cup):
    cups = play(cups, start_cup, 10000000)
    return cups[1] * cups[cups[1]]


def cups_from_raw(cups_raw):
    cups = [0] * (max(cups_raw) + 1)
    for i in range(len(cups_raw)):
        cups[cups_raw[i]] = cups_raw[(i + 1) % len(cups_raw)]

    return cups


def main():
    cups_raw = list(map(int, list('364297581')))

    starting_cup = cups_raw[0]
    cups1 = cups_from_raw(cups_raw)

    c = max(cups_raw)
    while c < 1000000:
        c += 1
        cups_raw.append(c)

    cups2 = cups_from_raw(cups_raw)

    print(f'\rPart One: {part_one(cups1, starting_cup)}')
    print(f'\rPart Two: {part_two(cups2, starting_cup)}')


if __name__ == '__main__':
    main()
