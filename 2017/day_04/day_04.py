import sys

valid_1, valid_2 = 0, 0

for line in sys.stdin:
    valid_words_1 = set()
    valid_words_2 = set()
    for word in line.split():
        valid_words_1.add(word)
        word = ''.join(sorted(word))
        valid_words_2.add(word)

    if len(valid_words_1) == len(line.split()):
        valid_1 += 1
    if len(valid_words_2) == len(line.split()):
        valid_2 += 1

print('Part One: {}\nPart Two: {}'.format(valid_1, valid_2))
