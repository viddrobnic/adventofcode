import sys
from collections import deque

programs = set()  # Set of all programs
children = set()  # Set of programs that are children
stack = dict()  # Describe the stack (key is parent, value is array of children)
weights = dict()  # Desribe the wights (key is program, value is weight)

for line in sys.stdin:  # Read the input
    name = line.split()[0]
    programs.add(name)  # Save name
    weights[name] = int(line.split()[1][1:-1])  # Save weight

    child = line.split('->')
    if len(child) > 1:  # If contains children append them
        child_array = child[1].replace(',', '').split()  # Remove commas and split
        stack[name] = child_array

        for c in child_array:
            children.add(c)  # Add children to children set
    else:  # If program has no children describe it in stack with empty arary
        stack[name] = []

root = list(programs - children)[0]  # Get root program


def different(array):  # Find different value in array
    values = set()  # Possible values
    for i in array:
        values.add(i)

    count_1, count_2 = 0, 0  # There should be 2 different values. Cound number of instances of each one
    values = list(values)
    for i in array:  # Count number of instances of each value
        count_1 += 1 if values[0] == i else 0
        count_2 += 1 if values[1] == i else 0

    if count_1 == 1:  # If values[0] appeared one time return the value and other values
        return values[0], values[1]
    else:
        return values[1], values[0]  # Values[1] appeared one time return the value and other values


def tower(program):  # Go recursivly throught the tower and find the different stack
    subtowers = stack[program]  # Get array of children
    w = weights[program]  # Get weight of program
    if len(subtowers) == 0:  # If no children return only the current weight
        return (False, w)

    tower_weights = []
    for i in subtowers:  # Append weights of towers
        t_weight = tower(i)
        if t_weight[0]:
            return t_weight

        tower_weights.append(t_weight[1])

    valid = True  # Check if weights are valid
    for i in range(len(tower_weights)):
        if tower_weights[i] != tower_weights[0]:
            valid = False
            break

    if not valid:
        diff, normal = different(tower_weights)  # Find the different index
        delta = normal - diff  # Calculate delta
        diff_index = tower_weights.index(diff)  # Find the different tower
        changed_weight = weights[subtowers[diff_index]] + delta
        return (True, changed_weight)  # Return True to indicate the wrong tower has been found and fixed
    else:
        return (False, w + sum(tower_weights))  # Return the weight of the current tower

changed_weight = tower(root)[1]

print('Part One: {}\nPart Two: {}'.format(root, changed_weight))
