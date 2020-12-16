def read_data():
    rules = dict()
    your_ticket = None
    nearby_tickets = []
    state = 0
    with open('in') as f:
        for line in map(lambda x: x.strip(), f.readlines()):
            if line == '':
                state += 1
                continue
            if line == 'your ticket:':
                continue
            if line == 'nearby tickets:':
                continue

            if state == 0:
                parts = line.split(':')
                ranges = parts[1].split('or')
                rules[parts[0]] = []
                for r in ranges:
                    nums = r.split('-')
                    rules[parts[0]].append((int(nums[0]), int(nums[1])))
            if state == 1:
                your_ticket = list(map(int, line.split(',')))

            if state == 2:
                nearby_tickets.append(list(map(int, line.split(','))))

    return rules, your_ticket, nearby_tickets


def part_one(rules, tickets):
    error_rate = 0
    invalid_tickets = []

    for i, ticket in enumerate(tickets):
        for val in ticket:
            valid_val = False
            for rule in rules.keys():
                valid = False
                for m, M in rules[rule]:
                    if val >= m and val <= M:
                        valid = True
                        break

                if valid:
                    valid_val = True
                    break

            if not valid_val:
                error_rate += val
                invalid_tickets.append(i)

    return invalid_tickets, error_rate


def part_two(rules, your_ticket, tickets):
    possible_values = [set(rules.keys()) for i in range(len(rules.keys()))]

    for ticket in tickets:
        for i, val in enumerate(ticket):
            for rule in rules.keys():
                valid = False
                for m, M in rules[rule]:
                    if val >= m and val <= M:
                        valid = True
                        break

                if not valid and rule in possible_values[i]:
                    possible_values[i].remove(rule)

    while True:
        to_filter = False
        for v in map(len, possible_values):
            if v > 1:
                to_filter = True
        if not to_filter:
            break

        for i in range(len(possible_values)):
            if len(possible_values[i]) == 1:
                for j in range(len(possible_values)):
                    if i != j:
                        possible_values[j] -= possible_values[i]

    res = 1
    for i in range(len(possible_values)):
        if 'departure' in list(possible_values[i])[0]:
            res *= your_ticket[i]

    return res


def main():
    rules, your_ticket, nearby_tickets = read_data()

    invalid_tickets, error_rate = part_one(rules, nearby_tickets)
    p_two = part_two(rules, your_ticket, [t for i, t in enumerate(nearby_tickets) if i not in invalid_tickets])

    print(f'Part One: {error_rate}')
    print(f'Part Two: {p_two}')


if __name__ == '__main__':
    main()
