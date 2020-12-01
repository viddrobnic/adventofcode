from queue import Queue

seed = 1362
seen = set()

def is_empty(x, y):
    n = x*x + 3*x + 2*x*y + y + y*y + seed

    return bin(n).count('1') % 2 == 0


def valid_moves(x, y):
    result = []
    actions = [-1, 1]
    for action in actions:
        new_x = x + action
        if x > 0 and is_empty(new_x, y) and (new_x, y) not in seen:
            result.append((new_x, y))

        new_y = y + action
        if y > 0 and is_empty(x, new_y) and (x, new_y) not in seen:
            result.append((x, new_y))

    return result


state = {
    'coords': (1, 1),
    'moves': 0
}

que = Queue()
que.put(state)
locations = 0
solved_1 = False
solved_2 = False

while not solved_1 or not solved_2:
    current_state = que.get()
    moves = current_state['moves']

    if current_state['coords'] in seen:
        continue
    seen.add(current_state['coords'])

    if current_state['coords'] == (31, 39):
        solved_1 = True
        print('#1:', moves)

    possible_moves = valid_moves(*current_state['coords'])
    for move in possible_moves:
        new_state = {'coords': move, 'moves': moves + 1}
        que.put(new_state)

    if moves <= 50:
        locations += 1
    else:
        solved_2 = True

print('#2:', locations)
