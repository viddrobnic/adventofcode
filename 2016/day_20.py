intervals = []
while True:
    try:
        a, b = input().split('-')
        intervals.append((int(a), int(b)))
    except:
        break

intervals.sort()

max_number = 0
i = 0
while intervals[i][0] <= max_number+1:
    if intervals[i][1] > max_number:
        max_number = intervals[i][1]
    i += 1

print(max_number, intervals[i], intervals[i-1])
print('#1:', max_number+1)


num_of_ips = 4294967295
count = 0
solved = False
i = 0

while not solved:
    max_number = intervals[i][1]
    while intervals[i][0] <= max_number:
        if intervals[i][1] > max_number:
            max_number = intervals[i][1]
        i += 1
        if i >= len(intervals):
            break

    if i < len(intervals):
        count += intervals[i][0] - max_number - 1
    else:
        count += num_of_ips - max_number
        solved = True

print('#2:', count)
