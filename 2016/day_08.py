display = [['.' for i in range(50)] for j in range(6)]

instructions = []
while True:
    try:
        instructions.append(input())
    except:
        break

for instruction in instructions:
    if instruction[:4] == 'rect':
        i, j = instruction.find(' '), instruction.find('x')
        width = int(instruction[i + 1: j])
        height = int(instruction[j+1:])
        for a in range(height):
            for b in range(width):
                display[a][b] = '#'
    if instruction[:13] == 'rotate column':
        i, j, k = instruction.find('='), instruction.rfind(' '), instruction.find(' by')
        column = int(instruction[i+1: k])
        step = int(instruction[j+1:])
        text = []
        for a in range(6):
            text.append(display[a][column])

        new_column = ['' for i in range(6)]
        for i in range(6):
            index = (i + step) % 6
            new_column[index] = text[i]
        for a in range(6):
            display[a][column] = new_column[a]
    if instruction[:10] == 'rotate row':
        i, j, k = instruction.find('='), instruction.rfind(' '), instruction.find(' by')
        row = int(instruction[i+1: k])
        step = int(instruction[j+1:])
        text = display[row]


        new_row = ['' for i in range(50)]
        for i in range(50):
            index = (i + step) % 50
            new_row[index] = text[i]
        display[row] = new_row[:]

count = 0
for row in display:
    for i in row:
        if i == '#':
            count += 1

print(count)
print()

for row in display:
    print(' '.join(row))
