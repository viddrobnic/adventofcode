directions = input().split(', ')

def point(n):
    start_point = [0, 0]
    rotation = 0

    for i in range(n):
        step = directions[i]
        if step[0] == 'L':
            rotation = (rotation - 1) % 4
        else:
            rotation = (rotation + 1) % 4
        move = int(step[1:])
        if rotation == 0:
            start_point[1] += move
        elif rotation == 1:
            start_point[0] += move
        elif rotation == 2:
            start_point[1] -= move
        elif rotation == 3:
            start_point[0] -=move

    return start_point

day_1 = point(len(directions))
print('#1:', abs(day_1[0]) + abs(day_1[1]))

for i in range(len(directions)):
    point_11 = point(i)
    point_12 = point(i + 1)
    horizontal_1 = point_11[1] == point_12[1]
    for j in range(i):
        point_21 = point(j)
        point_22 = point(j + 1)
        horizontal_2 = point_21[1] == point_22[1]
        if horizontal_1 == horizontal_2:
            continue

        if horizontal_1 and point_21[0] > min(point_11[0], point_12[0]) and point_21[0] < max(point_11[0], point_12[0]) and point_11[1] > min(point_21[1], point_22[1]) and point_11[1] < max(point_21[1], point_22[1]):
            print('#2:', abs(point_21[0]) + abs(point_11[1]))
            exit(0)
        elif horizontal_2 and point_11[0] > min(point_21[0], point_22[0]) and point_11[0] < max(point_21[0], point_22[0]) and point_21[1] > min(point_11[1], point_12[1]) and point_21[1] < max(point_11[1], point_12[1]):
            print('#2:', abs(point_21[1]) + abs(point_11[0]))
            exit(0)
