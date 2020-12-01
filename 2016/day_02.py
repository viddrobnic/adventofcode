instructions = []
while True:
    try:
        instructions.append(input())
    except:
        break

password_1 = []
password_2 = []
keypad_1 = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']]
keypad_2 =[['0', '0', '1', '0', '0'], ['0', '2', '3', '4', '0'], ['5', '6', '7', '8', '9'], ['0', 'A', 'B', 'C', '0'], ['0', '0', 'D', '0', '0']]

point_1 = [1, 1]
point_2 = [2, 0]
for line in instructions:
    for char in line:
        if char == 'U' and point_1[0] > 0:
            point_1[0] -= 1
        elif char == 'D' and point_1[0] < 2:
            point_1[0] += 1
        elif char == 'R' and point_1[1] < 2:
            point_1[1] += 1
        elif char == 'L' and point_1[1] > 0:
            point_1[1] -= 1
        
        if char == 'U' and point_2[0] > 0:
            new_point = point_2[:]
            new_point[0] -= 1
            if keypad_2[new_point[0]][new_point[1]] != '0':
                point_2 = new_point[:]
        elif char == 'D' and point_2[0] < 4:
            new_point = point_2[:]
            new_point[0] += 1
            if keypad_2[new_point[0]][new_point[1]] != '0':
                point_2 = new_point[:]
        elif char == 'R' and point_2[1] < 4:
            new_point = point_2[:]
            new_point[1] += 1
            if keypad_2[new_point[0]][new_point[1]] != '0':
                point_2 = new_point[:]
        elif char == 'L' and point_2[1] > 0:
            new_point = point_2[:]
            new_point[1] -= 1
            if keypad_2[new_point[0]][new_point[1]] != '0':
                point_2 = new_point[:]

    password_1.append(keypad_1[point_1[0]][point_1[1]])
    password_2.append(keypad_2[point_2[0]][point_2[1]])


print('#1:', ''.join(password_1))
print('#2:', ''.join(password_2))
