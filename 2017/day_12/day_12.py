import sys
from collections import defaultdict, deque

graph = defaultdict(set)  # Keys are vertices, values are sets of connections

for line in sys.stdin:
    line = line.replace(',', '')
    line = line.split()
    vertices = line[2:]
    graph[line[0]] |= set(vertices)
    for vertex in vertices:
        graph[vertex].add(line[0])

que = deque()
groups = 0
count_programs = 0
seen = set()

que.append('0')  # Start at program 0
seen.add('0')
while len(que) > 0:  # BFS through graph and count programs
    program = que.popleft()
    count_programs += 1
    for vertex in graph[program]:
        if vertex in seen:
            continue
        seen.add(vertex)
        que.append(vertex)

seen = set()  # Reset seen set
for program in graph.keys():  # For each unvisited program go through graph with BFS
    if program in seen:
        continue
    groups += 1  # If program not yet seen, it's its own separate group
    seen.add(program)
    que.append(program)
    while len(que) > 0:  # BFS
        p = que.popleft()
        for vertex in graph[p]:
            if vertex in seen:
                continue
            seen.add(vertex)
            que.append(vertex)

print('Part One: {}\nPart Two: {}'.format(count_programs, groups))
