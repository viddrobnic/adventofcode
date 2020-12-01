import sys

data = {}
m = 0
for line in sys.stdin:
    line = line.replace(':', '')
    line = list(map(int, line.split()))
    data[line[0]] = line[1]
    m = line[0]

m += 1

severity = 0
for i in range(m):
    if i not in data:
        continue
    if i % (2 * (data[i] - 1)) == 0:
        severity += i * data[i]

delta = 0
while True:
    caught = False
    for i in range(m):
        if i not in data:
            continue
        if (i + delta) % (2 * (data[i] - 1)) == 0:
            caught = True
            break

    if not caught:
        break
    delta += 1

print('Part One: {}\nPart Two: {}'.format(severity, delta))
