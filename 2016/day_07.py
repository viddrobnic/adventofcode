import re

data = []
while True:
    try:
        data.append(input())
    except:
        break


def find_aba(string):
    aba = set()
    for i in range(len(string) - 2):
        if string[i] == string[i+2] and string[i] != string[i+1]:
            aba.add(string[i:i+3])
    return aba


def find_bab(string):
    aba = find_aba(string)
    bab = set()
    for a in aba:
        bab.add(a[1] + a[0] + a[1])
    return bab


count_1 = 0
count_2 = 0

for line in data:
    outside_bracket = re.split(r'\[.*?\]', line)
    inside_bracket = re.findall(r'\[(.*?)\]', line)

    # part 2
    bab = set()
    aba = set()

    for string in inside_bracket:
        bab |= find_bab(string)

    for string in outside_bracket:
        aba |= find_aba(string)

    if len(aba & bab) > 0:
        count_2 += 1

    # part 1
    found = False
    for string in inside_bracket:
        if re.match(r'.*(\w)(?!\1)(\w)\2\1.*', string) is not None:
            found = True
            break
    if found:
        continue

    found = False
    for string in outside_bracket:
        if re.match(r'.*(\w)(?!\1)(\w)\2\1.*', string) is not None:
            found = True
            break
    if found:
        count_1 += 1


print('#1:', count_1)
print('#2:', count_2)
