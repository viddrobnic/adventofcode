from queue import Queue
import hashlib

passcode = 'pxxbnzuo'


def open(char):
    if char == 'b' or char == 'c' or char == 'd' or char == 'e' or char == 'f':
        return True
    return False


def possible_moves(path, position):
    res = []
    h = hashlib.md5((passcode + path).encode('utf8')).hexdigest()
    up, down, left, right = h[:4]
    if open(up) and position[1] > 0:
        res.append((path + 'U', (position[0], position[1] - 1)))
    if open(down) and position[1] < 3:
        res.append((path + 'D', (position[0], position[1] + 1)))
    if open(left) and position[0] > 0:
        res.append((path + 'L', (position[0] - 1, position[1])))
    if open(right) and position[0] < 3:
        res.append((path + 'R', (position[0] + 1, position[1])))

    return res


solved_1 = False
max_path = 0

queue = Queue()
queue.put(('', (0, 0)))
while not queue.empty():
    path, position = queue.get()
    if position == (3, 3):
        if not solved_1:
            print('#1:', path)
            solved_1 = True
        max_path = max(max_path, len(path))
        continue

    moves = possible_moves(path, position)
    for move in moves:
        queue.put(move)

print('#2:', max_path)
