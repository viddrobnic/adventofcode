commands = []
while True:
    try:
        commands.append(input())
    except:
        break

values = {'a': 0, 'b': 0, 'c': 1, 'd': 0}

i = 0
while i < len(commands):
    command = commands[i].split()

    if command[0] == 'cpy':
        from_reg = command[1]
        to_reg = command[2]

        try:
            values[to_reg] = int(from_reg)
        except:
            values[to_reg] = values[from_reg]
        i += 1
        continue

    if command[0] == 'inc':
        from_reg = command[1]
        values[from_reg] += 1
        i += 1
        continue

    if command[0] == 'dec':
        from_reg = command[1]
        values[from_reg] -= 1
        i += 1
        continue

    if command[0] == 'jnz':
        from_reg = command[1]
        to_reg = command[2]

        try:
            x = int(from_reg)
        except:
            x = values[from_reg]

        try:
            y = int(to_reg)
        except:
            y = values[to_reg]

        if x != 0:
            i += y
        else:
            i += 1

        continue

print(values)
