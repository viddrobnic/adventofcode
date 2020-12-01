from collections import defaultdict
import sys
import copy


# Read input data
graph = defaultdict(set)
dependencies = defaultdict(set)
visited = defaultdict(bool)
for line in sys.stdin:
    words = line.strip().split()
    edge1, edge2 = words[1], words[-3]
    graph[edge1].add(edge2)
    dependencies[edge2].add(edge1)

# Part 1
# Set initial available vertices
available = set()
for vertex in graph.keys():
    if len(dependencies[vertex]) != 0:
        continue

    available.add(vertex)

res = ''
# Do while you can (while vertices are available)
while len(available) != 0:
    # Sort available vertices, because we have to start
    # with the smalles according to alphabetic sort
    available_s = sorted(available)

    # Find next vertex, that is, vertex
    # with all dependencies staisfied
    next_v = None
    for vertex in available_s:
        possible = True

        deps = dependencies[vertex]
        for dep in deps:
            if not visited[dep]:
                possible = False
                break

        if possible:
            next_v = vertex
            break

    # If there is no next vertex something
    # went terribly wrong, so crash
    if next_v is None:
        raise Exception('vertex not found')

    res += next_v
    visited[next_v] = True
    available |= graph[next_v]
    available.remove(next_v)

print(f'Part 1: {res}')


# Part 2
# Empty visited, and create initial available vertices
visited = defaultdict(bool)
available = set()
for vertex in graph.keys():
    if len(dependencies[vertex]) != 0:
        continue

    available.add(vertex)

# Final time to output
res_time = 0

# Some constants
NUMBER_OF_WORKERS = 5
ADD_TIME = 60
# Array of current workers.
# i-th worker is working on workers_letters[i] letter,
# and needs workers_time[i] time to complete it
workers_letters = []
workers_time = []

# Do while workers are working and there are still letters available
while len(available) != 0 or len(workers_letters) != 0:
    # Clean workers that are finished (their time remaining is 0)
    i = 0
    while i < len(workers_time):
        if workers_time[i] == 0:
            workers_time.pop(i)

            # Add vertex as finished
            letter = workers_letters[i]
            visited[letter] = True
            workers_letters.pop(i)

            # Append new available nodes
            available |= graph[letter]
        else:
            i += 1

    # Find all available vertices that
    # have dependencies completed
    available_s = sorted(available)
    next_v = []
    for vertex in available_s:
        possible = True

        deps = dependencies[vertex]
        for dep in deps:
            if not visited[dep]:
                possible = False
                break

        if possible:
            next_v.append(vertex)

    # Give workers something to work on
    i = 0
    while len(workers_letters) < NUMBER_OF_WORKERS:
        if i >= len(next_v):
            break

        letter = next_v[i]

        workers_letters.append(letter)

        time = ADD_TIME + ord(letter) - ord('A') + 1
        workers_time.append(time)

        available.remove(letter)

        i += 1

    # If no workers are working on stuff, we are in inital
    # loop and no time has passed
    if len(workers_time) == 0:
        time_diff = 0
    else:  # When the first worker can finish
        time_diff = min(workers_time)

    # Add to time, and remove the passed time
    # from workers remaining time
    res_time += time_diff
    for i in range(len(workers_time)):
        workers_time[i] -= time_diff

print(f'Part 2: {res_time}')

