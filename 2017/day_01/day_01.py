l = list(map(int, input()))
s_1, s_2 = 0, 0
for i in range(len(l)):
    s_1 += l[i] if l[i] == l[(i + 1) % len(l)] else 0
    s_2 += l[i] if l[i] == l[(i + len(l) // 2) % len(l)] else 0
print('Part One: {}\nPart Two: {}'.format(s_1, s_2))
