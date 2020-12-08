def clean_line(line):
    instruction, argument = line.strip().split()
    return instruction, int(argument)


with open('in') as f:
    data = list(map(clean_line, f.readlines()))


def exec_code(program):
    acc = 0
    index = 0

    executed = [False] * len(data)

    while not executed[index]:
        executed[index] = True

        inst, arg = program[index]

        if inst == 'nop':
            index += 1
        elif inst == 'acc':
            acc += arg
            index += 1
        elif inst == 'jmp':
            index += arg

        if index >= len(program):
            return acc, True

    return acc, False


def part_one():
    acc, code = exec_code(data)
    return acc


def part_two():
    for i in range(len(data)):
        if data[i][0] == 'jmp':
            program = data[:]
            program[i] = ('nop', data[i][1])
            acc, code = exec_code(program)
            if code:
                return acc
        elif data[i][0] == 'nop':
            program = data[:]
            program[i] = ('jmp', data[i][1])
            acc, code = exec_code(program)
            if code:
                return acc


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
