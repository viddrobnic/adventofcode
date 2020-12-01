import copy
from queue import Queue

used = []
avail = []

nodes = {}

i = 0
while True:
    i += 1
    if i <= 2:
        input()
        continue
    try:
        line = input().split()
        name = line[0]
        us = int(line[2][:-1])
        av = int(line[3][:-1])
        node = (name, us, av)
        used.append(node)
        avail.append(node)

        storage = int(line[1][:-1])
        coords = name.split('-')
        x = int(coords[1][1:])
        y = int(coords[2][1:])
        node = {
            'storage': storage,
            'available': av,
            'used': us,
            'data_name': name
        }
        nodes[(x, y)] = node
    except:
        break

used.sort(key=lambda x: x[1])
avail.sort(key=lambda x: x[2], reverse=True)

pairs = set()

for used_node in used:
    if used_node[1] == 0:
        continue
    for avail_node in avail:
        if used_node[0] == avail_node[0]:
            continue
        if used_node[1] <= avail_node[2]:
            pair = tuple(sorted((used_node, avail_node), key=lambda x: x[0]))
            pairs.add(pair)
        else:
            break

print('#1:', len(pairs))

# for part 2 just print the grid and solve it by hand...
# you might want to adjust the numbers first
def print_grid(nodes):
    for y in range(30):
        for x in range(34):
            if nodes[(x, y)]['used'] == 0:
                print('_ ', end='')
            elif nodes[(x, y)]['used'] > 91:
                print('# ', end='')
            else:
                print('. ', end='')
        print()

print_grid(nodes)
