import copy
from collections import deque

seen = set()
elevator = 0

def is_valid(state, elevator):
    if elevator < 0 or elevator > 3:
         return False

    if (str(state), elevator) in seen:
        return False
    else:
        seen.add((str(state), elevator))

    no_pair_chip = []
    for pair in state:
        if pair[0] == pair[1]:
            continue
        no_pair_chip.append(pair[0])

    for pair in state:
        if pair[1] in no_pair_chip:
            return False

    return True


def valid_states(state, elevator):
    result = []
    for i in range(len(state)):
        for j in range(2):
            if state[i][j] == elevator:
                up = copy.deepcopy(state)
                up[i][j] += 1
                up.sort()
                if is_valid(up, elevator + 1):
                    result.append((up, elevator + 1))

                down = copy.deepcopy(state)
                down[i][j] -= 1
                down.sort()
                if is_valid(down, elevator-1):
                    result.append((down, elevator-1))

    for i in range(len(state)):
        for k1 in range(2):
            for j in range(i, len(state)):
                for k2 in range(2):
                    if i == j and k1 == k2:
                        continue
                    if state[i][k1] == elevator and state[j][k2] == elevator:
                        up = copy.deepcopy(state)
                        up[i][k1] += 1
                        up[j][k2] += 1
                        up.sort()
                        if is_valid(up, elevator+1):
                            result.append((up, elevator+1))

                        down = copy.deepcopy(state)
                        down[i][k1] -= 1
                        down[j][k2] -= 1
                        down.sort()
                        if is_valid(down, elevator-1):
                            result.append((down, elevator-1))
    return result


def moves(state):
    que = deque()
    que.append(state)
    while len(que) > 0:
        current_state = que.popleft()
        top_floor = True
        for item in current_state['items']:
            if item[0] != 3 or item[1] != 3:
                top_floor = False
                break

        if top_floor:
            return current_state['moves']

        possible_states = valid_states(current_state['items'], current_state['elevator'])
        for state in possible_states:
            que.append({'items': state[0], 'elevator': state[1], 'moves': current_state['moves'] + 1})


state_1 = {
    'items': [[1, 0], [0, 0], [1, 0], [0, 0], [0, 0]],
    'moves': 0,
    'elevator': 0
}

state_2 = {
    'items': [[1, 0], [0, 0], [1, 0], [0, 0], [0, 0], [0, 0], [0, 0]],
    'moves': 0,
    'elevator': 0
}

print('#1:', moves(state_1))
print('#2:', moves(state_2))
