from queue import Queue
from collections import defaultdict, deque
import copy

maze = []
while True:
    try:
        maze.append(list(input()))
    except:
        break

num_of_checkpoints = 0

for y in range(len(maze)):
    for x in range(len(maze[y])):
        c = maze[y][x]
        if c in '123456789':
            maze[y][x] = int(c)
            num_of_checkpoints += 1
        elif c == '0':
            c = '.'
            start = (x, y)

        if c == '.':
            maze[y][x] = 0
        elif c == '#':
            maze[y][x] = -1


for _ in range(10):
    for y in range(len(maze)):
        for x in range(len(maze[y])):
            if maze[y][x] == 0 and (x, y) != start:
                count = 0
                if maze[y-1][x] == -1:
                    count += 1
                if maze[y+1][x] == -1:
                    count += 1
                if maze[y][x-1] == -1:
                    count += 1
                if maze[y][x+1] == -1:
                    count += 1
                if count >= 3:
                    maze[y][x] = -1

target_checkpoints = 0
for i in range(num_of_checkpoints):
    target_checkpoints |= (1 << i)


visited = defaultdict(set)
start_2 = []
solved_first = False

original_start = start

que = deque()
que.append((*start, 0, 0))
while que:
    x, y, moves, checkpoints = que.popleft()

    if maze[y][x] > 0:
        checkpoints |= (1 << (maze[y][x]-1))
        if checkpoints == target_checkpoints:
            start_2.append((x, y, moves))
            if not solved_first:
                print('#1:', moves)
                solved_first = True


    if checkpoints in visited[(x, y)]:
        continue
    visited[(x, y)].add(checkpoints)

    if maze[y][x-1] != -1:
        que.append((x-1, y, moves+1, checkpoints))
    if maze[y][x+1] != -1:
        que.append((x+1, y, moves+1, checkpoints))
    if maze[y-1][x] != -1:
        que.append((x, y-1, moves+1, checkpoints))
    if maze[y+1][x] != -1:
        que.append((x, y+1, moves+1, checkpoints))



visited = defaultdict(bool)
que = deque()

for start in start_2:
    que.append(start)

while que:
    x, y, moves = que.popleft()

    if (x, y) == original_start:
        print('#2:', moves)
        break

    if visited[(x, y)]:
        continue
    visited[(x, y)] = True

    if maze[y][x-1] != -1:
        que.append((x-1, y, moves+1))
    if maze[y][x+1] != -1:
        que.append((x+1, y, moves+1))
    if maze[y-1][x] != -1:
        que.append((x, y-1, moves+1))
    if maze[y+1][x] != -1:
        que.append((x, y+1, moves+1))
