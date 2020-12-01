import collections

lines = []

while True:
    try:
        lines.append(input())
    except:
        break

message_1 = ''
message_2 = ''

for i in range(len(lines[0])):
    line = ''
    for j in range(len(lines)):
        line += lines[j][i]
    coll = collections.Counter(line)
    message_1 += coll.most_common(1)[0][0]
    message_2 += coll.most_common()[-1][0]

print('#1:', message_1)
print('#2:', message_2)
