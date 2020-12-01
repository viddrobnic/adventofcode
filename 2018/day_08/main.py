import sys
from collections import defaultdict, deque


# Read input
data = list(map(int, input().split()))


# Class tree, to which the input is parsed
class Tree:
    def __init__(self, children, metadata):
        self.children = children
        self.metadata = metadata


# Parse tree, starting at index i
def parse_tree(i):
    nr_children, nr_metadata = data[i], data[i + 1]

    tree = Tree([], [])

    # First tree starts at index i + 1
    j = i + 1
    # Parse all children
    for _ in range(nr_children):
        child, j = parse_tree(j + 1)
        tree.children.append(child)

    # Add metadata to tree
    for m in range(j + 1, j + 1 + nr_metadata):
        tree.metadata.append(data[m])

    # Return tree, and index at which the tree ends
    return tree, j + nr_metadata

# Final tree
tree, _ = parse_tree(0)

# Part 1
# BFS throgh the tree
res = 0
que = deque()
que.append(tree)
while len(que) > 0:
    stick = que.popleft()
    for child in stick.children:
        que.append(child)

    res += sum(stick.metadata)

print(f'Part 1: {res}')


# Part 2
def value_of_tree(tree):
    if len(tree.children) == 0:
        return sum(tree.metadata)

    res = 0
    for i in tree.metadata:
        index = i - 1
        if index >= len(tree.children) or index < 0:
            continue

        res += value_of_tree(tree.children[index])

    return res


res = value_of_tree(tree)
print(f'Part 2: {res}')

