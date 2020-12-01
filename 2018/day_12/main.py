import sys


# Read input
initial_state = input().split()[-1]
initial_state = list(initial_state)
# Add some space to the left and right of the initial state
n = len(initial_state) * 10
initial_state = ['.'] * n + initial_state + ['.'] * n

# Read empty line
input()
# Read the rules for the plants growth
growth = {}
for i in sys.stdin:
    key, value = i.strip().split(' => ')
    growth[key] = value


# Function for calculating how many plants there are in
# the given state
def calculate_plants(state):
    res = 0
    for i in range(len(state)):
        if state[i] == '#':
            res += i - n

    return res

def generate_new_state(state):
    new_state = state[:]
    for i in range(2, len(state) - 2):
        new = ''.join(state[i - 2 : i + 3])
        new_state[i] = growth[new]

    return new_state


# Part 1
state = initial_state[:]
for _ in range(20):
    state = generate_new_state(state)

part_1 = calculate_plants(state)
print(f'Part 1: {part_1}')

# Part 2
# If we look at what we are getting, we notice that evntually (around 200th iteration),
# the plants are in the same pattern, but they are moving to the right. They move
# one place to the right every iteration. With that knowledge, we can calculate the
# result without iterating through all 50 billion or whatever the number is iterations.
state = initial_state[:]
for _ in range(200):
    state = generate_new_state(state)

number_of_plants = sum(map(lambda x: x == '#', state))
part_2 = calculate_plants(state) + (50000000000 - 200) * number_of_plants
print(f'Part 2: {part_2}')

