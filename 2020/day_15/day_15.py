from collections import defaultdict

starting_numbers = [16, 12, 1, 0, 15, 7, 11]


def solver(rounds):
    last_spoken = dict()
    number_spoken = defaultdict(int)
    for i, n in enumerate(starting_numbers):
        last_spoken[n] = i + 1
        number_spoken[n] += 1

    most_recent = starting_numbers[-1]
    turn = len(starting_numbers)
    while turn != rounds:
        turn += 1

        prev_most_recent = most_recent

        if number_spoken[most_recent] <= 1:
            most_recent = 0
        else:
            most_recent = turn - 1 - last_spoken[most_recent]

        number_spoken[most_recent] += 1
        last_spoken[prev_most_recent] = turn - 1

    return most_recent


if __name__ == '__main__':
    print(f'Part One: {solver(2020)}')
    print(f'Part Two: {solver(30000000)}')
