data = list(map(int, input().split()))

seen = set()  # States already generated
seen.add(tuple(data))
indexes = {}  # For each state save when it has been seen

cycles = 0  # Number of cycles before they start repeating
loop_size = 0  # Size of the lopp

while True:
    start = data.index(max(data))  # Start index
    a = data[start]  # Number at start
    data[start] = 0  # Reset the index
    for i in range(a):
        data[(start+i+1) % len(data)] += 1  # Redestribute nubmers

    cycles += 1

    if tuple(data) in seen:  # If seen calculate loop size and exit
        old_index = indexes[tuple(data)]
        loop_size = cycles - old_index
        break
    else:  # If not seen store state
        seen.add(tuple(data))
        indexes[tuple(data)] = cycles

print('Part One: {}\nPart Two: {}'.format(cycles, loop_size))
