with open('in') as f:
    data = list(map(lambda l: list(l.strip().replace(' ', '')), f.readlines()))


def evaluate(e):
    value = None
    operation = None

    i = 0
    while i < len(e):
        c = e[i]

        if c in '+*':
            operation = c
        elif c in '0123456789':
            c = int(c)
            if operation is None:
                value = c
            else:
                if operation == '+':
                    value += c
                else:
                    value *= c
        elif c == '(':
            tmp_e = []
            i += 1

            level = 1
            while True:
                if e[i] == '(':
                    level += 1
                elif e[i] == ')':
                    level -= 1

                if level == 0:
                    break
                else:
                    tmp_e.append(e[i])

                i += 1

            b = evaluate(tmp_e)
            if operation is None:
                value = b
            else:
                if operation == '+':
                    value += b
                else:
                    value *= b
        else:
            assert False

        i += 1

    return value


def evaluate_2(e):
    i = 0
    while i < len(e):
        if e[i] == '+':
            level = 0
            index = -1
            for j in range(i - 1, -1, -1):
                if e[j] == ')':
                    level += 1
                elif e[j] == '(':
                    level -= 1

                if (level == 0 and e[j] in '+*') or (level < 0 and e[j] in '()'):
                    index = j
                    break
            e.insert(index + 1, '(')
            i += 1

            level = 0
            index = len(e)
            for j in range(i + 1, len(e)):
                if e[j] == '(':
                    level += 1
                elif e[j] == ')':
                    level -= 1

                if (level == 0 and e[j] in '+*') or (level < 0 and e[j] in '()'):
                    index = j
                    break
            e.insert(index, ')')

        i += 1

    return evaluate(e)


def part_one():
    return sum(map(evaluate, data))


def part_two():
    return sum(map(evaluate_2, data))


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
