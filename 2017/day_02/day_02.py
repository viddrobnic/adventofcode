import sys

s_1, s_2 = 0, 0
for line in sys.stdin:
    i = list(map(int, line.split()))
    s_1 += max(i) - min(i)

    for a in range(len(i) - 1):
        for b in range(a + 1, len(i)):
            s_2 += i[a] // i[b] if i[a] % i[b] == 0 else 0
            s_2 += i[b] // i[a] if i[b] % i[a] == 0 else 0

print('Part One: {}\nPart Two: {}'.format(s_1, s_2))
