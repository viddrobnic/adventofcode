from collections import defaultdict
import sys

registers = defaultdict(int)
max_value = 0  # Maximum value that apperead (part two)

for line in sys.stdin:
    data = line.split()

    operation = 1 if data[1] == 'inc' else -1  # Add or substract

    number = int(data[2]) * operation  # Number to add or substract
    left = data[4]  # Left side of comparison
    right = int(data[6])  # Right side of comparison
    comparison = data[5]

    if_statement = 'registers["{}"] {} {}'.format(left, comparison, right)
    res = eval(if_statement)  # Execute if statemen
    if res:
        registers[data[0]] += number

    max_value = max(max_value, max(registers.values()))  # Set maximum value

print('Part One: {}\nPart Two: {}'.format(max(registers.values()), max_value))
