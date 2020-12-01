step = int(input())

spinlock = [0]
position = 0

for i in range(1, 2018):
    position = (position + step) % len(spinlock)
    position += 1
    spinlock.insert(position, i)

result_1 = spinlock[(position + 1) % len(spinlock)]

length = 1
position = 0
result_2 = 0
for i in range(1, 50000001):
    if i % 100000 == 0:
        progress = i / 500000
        number_of_bars = int(progress // 5)
        print('\rPart Two progress: \t[' + '#'*number_of_bars + ' '*(20 - number_of_bars) + ']\t{}%'.format(int(progress)) , end='')  # Some ugly code to make nice display of progress, because it takes so long

    position = (position + step) % length
    position += 1

    if position == 1:
        result_2 = i

    length += 1

print('\nPart One: {}\nPart Two: {}'.format(result_1, result_2))
