import sys

data = []
for line in sys.stdin:
    data.append(line)

y = 0
x = 0
for i in range(len(data[0])):  # Find starting point
    if data[0][i] == '|':
        x = i
        break

# Possible directions
directions = {
    'd': (0, 1),
    'u': (0, -1),
    'l': (-1, 0),
    'r': (1, 0),
}
d = 'd'  # My input starts with down, probably everyones does. If not change it.

letters = ''  # Part One solution
steps = 0  # Part Two solution

while True:  # Continue until at the end
    x_1 = x + directions[d][0]  # x after moving
    y_1 = y + directions[d][1]  # y after moving
    steps += 1  # You just took a step so count it!

    # Check that coordinates are not outside the map (probably never will be, but just to be sure)
    if x_1 >= 0 and x_1 < len(data[0]) and y_1 >= 0 and y_1 < len(data):
        if data[y_1][x_1] != ' ':  # If not void you can move there
            x = x_1  # Set new coordinates
            y = y_1

            # If no letter on this position just continue
            if data[y_1][x_1] == '|' or data[y_1][x_1] == '-' or data[y_1][x_1] == '+':
                continue
            else:  # It here is a letter here, remember it
                letters += data[y_1][x_1]
                continue
        else:  # If you can not move there
            if d == 'd' or d == 'u':  # If you were going down, you don't want to go up, and you can't go down, same for up
                check = ['l', 'r']
            if d == 'l' or d == 'r':  # Same logic for left and right
                check = ['u', 'd']

            found_next = False  # Found next position in the path
            for c in check:
                x_2 = x + directions[c][0]  # New coordinates
                y_2 = y + directions[c][1]
                if x_2 < 0 or x_2 >= len(data[0]) or y_2 < 0 or y_2 >= len(data):  # Check you are not outside the map
                    continue
                elif data[y_2][x_2] == ' ':  # If there is nothing here continue
                    continue
                else:  # Valid position, to where you want to move
                    if data[y_2][x_2] != '-' and data[y_2][x_2] != '|' and data[y_2][x_2] != '+':  # Check if is a letter
                        letters += data[y_2][x_2]  # If position contains a letter remember it
                    x = x_2  # Change coordinates
                    y = y_2
                    d = c  # Change the course
                    found_next = True  # You just found a valid position
                    break

            if not found_next:  # If there was no valid position, you are at the end
                break

print('Part One: {}\nPart Two: {}'.format(letters, steps))
