import re
from collections import defaultdict

rooms = []
while True:
    try:
        rooms.append(input())
    except:
        break


def decrypt(room, sector):
    global room_name
    name = ' '.join(room.split('-')[:-1])
    new_name = ''
    for i in range(sector):
        for j in range(len(name)):
            if name[j] != ' ':
                new_name += chr(((ord(name[j]) - ord('a') + 1) % 26) + ord('a'))
            else:
                new_name += ' '
        name = new_name
        new_name = ''
    if 'northpole' in name:
        room_name = sector


count_1 = 0
room_name = ''

for room in rooms:
    name = ''.join(room.split('-')[:-1])
    match = re.search(r'(\d+)\[(\w+)\]', room.split('-')[-1])
    sector = match.groups()[0]
    checksum = match.groups()[1]

    d = defaultdict(int)
    for letter in name:
        d[letter] += 1

    histogram = []
    for k, v in d.items():
        histogram.append((v, k))

    histogram.sort(key=lambda x: x[1])
    histogram.sort(key=lambda x: x[0], reverse=True)

    check = (''.join(i[1] for i in histogram))[:5]

    if check == checksum:
        count_1 += int(sector)
        decrypt(room, int(sector))

print('#1:', count_1)
print('#2:', room_name)
