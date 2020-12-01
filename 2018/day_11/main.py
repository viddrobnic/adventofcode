# Read input
serial_number = int(input())

# Create power grid
grid = [[None for i in range(300)] for j in range(300)]

for y in range(1, 301):
    for x in range(1, 301):
        rack_id = x + 10
        power = rack_id * y
        power += serial_number
        power *= rack_id
        power %= 1000
        power //= 100
        power -= 5
        grid[y - 1][x - 1] = power

# Calculate partial sums
partial_sums = [[None for i in range(300)] for j in range(300)]
for y in range(300):
    for x in range(300):
        partial_sums[y][x] = grid[y][x]
        if y == 0 and x == 0:
            pass
        elif y == 0:
            partial_sums[y][x] += partial_sums[y][x - 1]
        elif x == 0:
            partial_sums[y][x] += partial_sums[y - 1][x]
        else:
            partial_sums[y][x] =partial_sums[y][x] + partial_sums[y - 1][x] + partial_sums[y][x - 1] - partial_sums[y - 1][x - 1]


# Function for calculatin max power, given the square size
def max_power_for_size(size):
    offset = size - 1
    max_x, max_y, max_power = None, None, None
    for y in range(300 - size):
        for x in range(300 - size):
            power = partial_sums[y + offset][x + offset]
            if y == 0 and x == 0:
                pass
            elif y == 0:
                power -= partial_sums[y + offset][x - 1]
            elif x == 0:
                power -= partial_sums[y - 1][x + offset]
            else:
                power = power - partial_sums[y + offset][x - 1] - partial_sums[y - 1][x + offset] + partial_sums[y - 1][x - 1]

            if max_power is None or power > max_power:
                max_x, max_y, max_power = x, y, power

    return max_x + 1, max_y + 1, max_power


# Part 1
max_x, max_y, _ = max_power_for_size(3)
print(f'Part 1: {max_x},{max_y}')

# Part 2
max_x, max_y, max_size, max_power = None, None, None, None
for size in range(1, 300):
    x, y, power = max_power_for_size(size)

    if max_power is None or power > max_power:
        max_x, max_y, max_size, max_power = x, y, size, power

print(f'Part 2: {max_x},{max_y},{max_size}')

