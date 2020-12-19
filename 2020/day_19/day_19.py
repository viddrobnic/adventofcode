import re

rules = dict()
calculated_rules = dict()
messages = []

with open('in') as f:
    is_rule = True
    for l in f.readlines():
        l = l.strip()
        if l == '':
            is_rule = False
            continue

        if is_rule:
            parts = l.split()
            rule = []
            current_part = []
            for p in parts[1:]:
                if p == '|':
                    rule.append(current_part)
                    current_part = []
                else:
                    current_part.append(p)

            rule.append(current_part)
            rules[parts[0][:-1]] = rule
        else:
            messages.append(l)


def calculate_rule(r, star_part):
    if r in calculated_rules:
        return calculated_rules[r]

    rule = ''
    if star_part == 2 and r == '11':
        part1 = calculate_rule(rules[r][0][0], star_part)
        part2 = calculate_rule(rules[r][0][1], star_part)

        rule += '(?:'
        for i in range(1, 21):
            rule += part1 * i
            rule += part2 * i
            if i < 20:
                rule += '|'

        rule += ')'
        calculated_rules[r] = rule
        return rule

    if len(rules[r]) == 1:
        for subpart in rules[r][0]:
            if subpart[0] == '"':
                rule += subpart[1]
            else:
                rule += calculate_rule(subpart, star_part)
    else:
        rule += '(?:'
        for i, part in enumerate(rules[r]):
            for subpart in part:
                if subpart[0] == '"':
                    rule += subpart[1]
                else:
                    rule += calculate_rule(subpart, star_part)

            if i < len(rules[r]) - 1:
                rule += '|'

        rule += ')'

    if star_part == 2:
        if r == '8':
            rule += '+'

    calculated_rules[r] = rule
    return rule


def part_one():
    res = 0

    p = re.compile('^' + calculate_rule('0', 1) + '$')
    for m in messages:
        if p.match(m) is not None:
            res += 1

    return res


def part_two():
    global calculated_rules
    calculated_rules = dict()

    res = 0

    p = re.compile('^' + calculate_rule('0', 2) + '$')
    for m in messages:
        if p.match(m) is not None:
            res += 1

    return res


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
