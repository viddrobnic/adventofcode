import hashlib

seed = 'qzyelonm'


def matches_three(line):
    for i in range(len(line) - 2):
        matches = True
        for j in range(i+1, i+3):
            if line[i] != line[j]:
                matches = False
                break
        if matches:
            return line[i]
    return None


def matches_five(line):
    result = set()
    for i in range(len(line) - 4):
        matches = True
        for j in range(i+1, i+5):
            if line[i] != line[j]:
                matches = False
                break
        if matches:
            result.add(line[i])
    return list(result)


keys = []
valid_keys = []

i = 0
while len(valid_keys) <= 64:
    h = hashlib.md5((seed + str(i)).encode('utf8')).hexdigest()
    char = matches_three(h)
    keys.append((i, char))

    matches = matches_five(h)
    for match in matches:
        j = 0
        while j < len(keys):
            key = keys[j]
            if i - key[0] > 1000:
                keys.pop(j)
                continue

            if key[1] == match and key[0] != i:
                keys.pop(j)
                j -= 1
                valid_keys.append(key)
            j += 1
    i += 1

print('#1:', valid_keys[62][0])

keys = []
valid_keys = []
i = 0

while len(valid_keys) <= 64:
    h = hashlib.md5((seed + str(i)).encode('utf8')).hexdigest()
    for _ in range(2016):
        h = hashlib.md5(h.encode('utf8')).hexdigest()
    char = matches_three(h)
    keys.append((i, char))

    matches = matches_five(h)
    for match in matches:
        j = 0
        while j < len(keys):
            key = keys[j]
            if i - key[0] > 1000:
                keys.pop(j)
                continue

            if key[1] == match and key[0] != i:
                keys.pop(j)
                j -= 1
                valid_keys.append(key)
            j += 1
    i += 1

print('#2:', valid_keys[63][0])
