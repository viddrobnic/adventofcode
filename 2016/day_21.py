instructions = []
while True:
    try:
        instructions.append(input())
    except:
        break

def swap_position(password, sub):
    x = int(sub[2])
    y = int(sub[-1])
    a = password[x]
    password[x] = password[y]
    password[y] = a
    return password


def swap_letter(password, sub):
    x = sub[2]
    y = sub[-1]
    for i in range(len(password)):
        if password[i] == x:
            password[i] = y
        elif password[i] == y:
            password[i] = x
    return password


def reverse(password, sub):
    x = int(sub[2])
    y = int(sub[-1]) + 1
    return password[:x] + list(reversed(password[x:y])) + password[y:]


password = list('abcdefgh')
for instruction in instructions:
    sub = instruction.split()
    if sub[0] == 'swap' and sub[1] == 'position':
        password = swap_position(password, sub)
    elif sub[0] == 'swap' and sub[1] == 'letter':
        password = swap_letter(password, sub)
    elif sub[0] == 'rotate' and sub[1] != 'based':
        step = int(sub[-2]) % len(password)
        for i in range(step):
            if sub[1] == 'left':
                password.append(password.pop(0))
            else:
                password.insert(0, password.pop(-1))
    elif sub[0] == 'rotate' and sub[1] == 'based':
        x = sub[-1]
        index = password.index(x)
        if index >= 4:
            index += 1
        index += 1
        step = index % len(password)
        for i in range(step):
            password.insert(0, password.pop(-1))
    elif sub[0] == 'reverse':
        password = reverse(password, sub)
    elif sub[0] == 'move':
        x = int(sub[2])
        y = int(sub[-1])
        password.insert(y, password.pop(x))

print('#1:', ''.join(password))


password = list('fbgdceah')
instructions.reverse()
for instruction in instructions:
    sub = instruction.split()
    if sub[0] == 'swap' and sub[1] == 'position':
        password = swap_position(password, sub)
    elif sub[0] == 'swap' and sub[1] == 'letter':
        password = swap_letter(password, sub)
    elif sub[0] == 'rotate' and sub[1] != 'based':
        step = int(sub[-2]) % len(password)
        for i in range(step):
            if sub[1] == 'right':
                password.append(password.pop(0))
            else:
                password.insert(0, password.pop(-1))
    elif sub[0] == 'rotate' and sub[1] == 'based':
        x = sub[-1]
        orig_password = password[:]
        while True:
            password.append(password.pop(0))

            index = password.index(x)
            if index >= 4:
                index += 1
            index += 1
            step = index % len(password)
            new_pass = password[:]
            for i in range(step):
                new_pass.insert(0, new_pass.pop(-1))
            if new_pass == orig_password:
                break
    elif sub[0] == 'reverse':
        password = reverse(password, sub)
    elif sub[0] == 'move':
        y = int(sub[2])
        x = int(sub[-1])
        password.insert(y, password.pop(x))

print('#2:', ''.join(password))
