import sys
data = []
for line in sys.stdin:
    data.append(int(line))

data_2 = data[:]

steps_1 = 0
i = 0
while i < len(data):
    a = data[i]
    data[i] += 1
    i += a
    steps_1 += 1

steps_2 = 0
i = 0
while i < len(data_2):
    a = data_2[i]
    if a >= 3:
        data_2[i] -= 1
    else:
        data_2[i] += 1
    i += a
    steps_2 += 1

print('Part One: {}\nPart Two: {}'.format(steps_1, steps_2))
