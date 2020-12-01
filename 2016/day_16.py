start = '01111001100111011'

part_1 = start[:]
part_2 = start[:]

while len(part_1) < 35651584:
    a = part_1
    b = a[:]
    b = b[::-1]
    b = b.replace('1', 'a')
    b = b.replace('0', '1')
    b = b.replace('a', '0')
    part_1 = a + '0' + b
    part_2 = part_1[:]

def checksum(string):
    res = ''
    for i in range(0, len(string)-1, 2):
        if string[i] == string[i+1]:
            res += '1'
        else:
            res += '0'

    if len(res) % 2 == 0:
        return checksum(res)
    else:
        return res

part_1 = part_1[0:272]
part_2 = part_2[0:35651584]

print('#1:', checksum(part_1))
print('#2:', checksum(part_2))
