import sys


# Read data
data = []

for i in sys.stdin:
    clean = i.strip()
    if clean == '':
        break

    data.append(int(clean))

# Part 1
freq = 0
for i in data:
    freq += i
print(f'Part 1: {freq}')

# Part 2
freq = 0
running = True
seen = set()
while running:
    for i in data:
        freq += i
        if freq in seen:
            running = False
            break

        seen.add(freq)

print(f'Part 2: {freq}')

