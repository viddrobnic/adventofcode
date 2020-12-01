import sys
from collections import Counter

data = []

# Read data
for i in sys.stdin:
    cleaned = i.strip()
    data.append(cleaned)

# Part 1
two_letter, three_letter = 0, 0

for entry in data:
    count = Counter(entry)
    count = set(count.values())

    if 2 in count:
        two_letter += 1

    if 3 in count:
        three_letter += 1

checksum = two_letter * three_letter
print(f'Part 1: {checksum}')

# Part 2
for i in range(len(data)):
    done = False

    for j in range(i + 1, len(data)):
        id1 = data[i]
        id2 = data[j]

        number_of_diffs = 0
        diff_index = 0
        for k in range(len(id1)):
            if id1[k] != id2[k]:
                number_of_diffs += 1
                diff_index = k

        if number_of_diffs == 1:
            common_letters = id1[:diff_index] + id1[diff_index:]
            print(f'Part 2: {common_letters}')

            done = True
            break

    if done:
        break

