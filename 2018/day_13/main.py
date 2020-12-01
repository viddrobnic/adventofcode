import sys
import copy


# 0 = up
# 1 = down
# 2 = left
# 3 = right

# Some constants
MOVE = [(0, -1), (0, 1), (-1, 0), (1, 0)]
TURN_LEFT = [2, 3, 1, 0]
TURN_RIGHT = [3, 2, 0, 1]

data = []
carts = set()

# Read data
for line in sys.stdin:
    data.append(list(line[:-1]))

# Extract carts from data
for y in range(len(data)):
    row = data[y]
    for x in range(len(row)):
        elt = row[x]
        if elt == '^':
            carts.add((x, y, 0, 0))
            data[y][x] = '|'
        elif elt == 'v':
            carts.add((x, y, 1, 0))
            data[y][x] = '|'
        elif elt == '<':
            carts.add((x, y, 2, 0))
            data[y][x] = '-'
        elif elt == '>':
            carts.add((x, y, 3, 0))
            data[y][x] = '-'


# Function that prints the map
# Used for debugging purposes
def print_map():
    for y in range(len(data)):
        row = data[y]
        for x in range(len(row)):
            is_cart = False
            for cart in carts:
                if cart[0] == x and cart[1] == y:
                    print('#', end='')
                    is_cart = True
                    break
            if is_cart:
                continue

            print(row[x], end='')
        print()


part_1, part_2 = False, False
while not part_1 or not part_2:
    # Sort carts how the instructions say
    sorted_carts = list(sorted(carts, key=lambda x: (x[1], x[0])))

    for cart in sorted_carts:
        # If cart has been removed after a collision ignore it
        if cart not in carts:
            continue

        # Remove cart, because we are going to edit add
        carts.remove(cart)

        dx, dy = MOVE[cart[2]]
        x1, y1 = cart[0] + dx, cart[1] + dy

        if data[y1][x1] != '+':  # If we are not on crossroad just move it
            m = cart[2]
            # Check if there is a turn
            if data[y1][x1] == '/' or data[y1][x1] == '\\':
                # Set new direction according to the current direction
                # And where to turn
                if cart[2] == 0:
                    if data[y1][x1] == '/':
                        m = 3
                    else:
                        m = 2
                elif cart[2] == 1:
                    if data[y1][x1] == '/':
                        m = 2
                    else:
                        m = 3
                elif cart[2] == 2:
                    if data[y1][x1] == '/':
                        m = 1
                    else:
                        m = 0
                elif cart[2] == 3:
                    if data[y1][x1] == '/':
                        m = 0
                    else:
                        m = 1
            # Add cart again
            cart = (x1, y1, m, cart[3])
            carts.add(cart)
        else:
            # If we are on crossroad
            nr_turns = cart[3]
            if nr_turns == 0:
                turn = TURN_LEFT[cart[2]]
            elif nr_turns == 1:
                turn = cart[2]
            elif nr_turns == 2:
                turn = TURN_RIGHT[cart[2]]

            nr_turns = (nr_turns + 1) % 3

            # Add cart again
            cart = (x1, y1, turn, nr_turns)
            carts.add(cart)

        # Copy carts sowe can edit it while we iterate
        removed_carts = copy.deepcopy(carts)
        for cart1 in carts:
            # We don't want collisions with the same cart
            if cart1 == cart:
                continue

            # Check if collision
            if cart1[0] == cart[0] and cart1[1] == cart[1]:
                # Print the first collision coordinates
                if not part_1:
                    part_1 = True
                    print(f'Part 1: {cart[0]},{cart[1]}')

                removed_carts.remove(cart1)
                removed_carts.remove(cart)

        carts = removed_carts

    # Print last remaining cart
    if len(carts) == 1 and not part_2:
        cart = carts.pop()
        part_2 = True
        print(f'Part 2: {cart[0]},{cart[1]}')

    # print_map()

