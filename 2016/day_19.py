elves = [i for i in range(1, 3012210+1)]

remove_first = len(elves) % 2 != 0

while len(elves) > 1:
    elves = [j for i, j in enumerate(elves) if i % 2 == 0]
    if remove_first:
        elves.pop(0)
    remove_first = len(elves) % 2 != 0

print('#1:', elves[0])


elves = [i for i in range(1, 3012210+1)]

while len(elves) > 1:
    pop = set()
    last = 0
    for i in range(len(elves) // 2):
        last = elves[i]
        pop.add(elves[(i+(len(elves) + i) // 2) % len(elves)])

    elves = [i for i in elves if i not in pop]
    if len(elves) == 1:
        break

    pop = set()
    start = elves.index(last) + 1
    for i in range(start, len(elves)):
        pop.add(elves[(i + (len(elves) + i - start) // 2) % len(elves)])
    elves = [i for i in elves if i not in pop]

print('#2:', elves[0])
