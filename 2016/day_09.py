enc = input()

def solve(string, add_to_1):
    in_bracket = False
    pre_x = True
    a = ''
    b = ''
    length_1 = 0
    length_2 = 0
    i = 0

    while i < len(string):
        char = string[i]
        i += 1


        if char == '(':
            in_bracket = True
            continue
        if char == ')':
            in_bracket = False
            pre_x = True

            if add_to_1:
                length_1 += int(a) * int(b)
            length_2 += int(b) * solve(string[i: i + int(a)], False)[1]

            i += int(a)
            a = ''
            b = ''

            continue

        if not in_bracket:
            length_1 += 1
            length_2 += 1
            continue

        if char == 'x':
            pre_x = False
            continue

        if pre_x:
            a += char
        else:
            b += char

    return length_1, length_2

len_1, len_2 = solve(enc, True)
print('#1:', len_1)
print('#2:', len_2)
