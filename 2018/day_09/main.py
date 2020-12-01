from collections import defaultdict
import sys


# Class for creating linked list
class Node:
    def __init__(self, data=None, next=None, previous=None):
        self.data = data
        self.next = next
        self.previous = previous


# Read input
words = input().split()
players, max_marble = int(words[0]), int(words[-2])


# Calculate max score
def max_score(players, max_marble):
    current_elf = 0

    current_node = Node(data=0)
    current_node.previous = current_node
    current_node.next = current_node

    score = defaultdict(int)

    for marble in range(1, max_marble + 1):
        # Add marble on board
        if marble % 23 != 0:
            node = current_node.next

            new_node = Node(data=marble, previous=node, next=node.next)
            next_node = node.next
            node.next = new_node
            next_node.previous = new_node

            current_node = new_node
        # Add to score
        else:
            score[current_elf] += marble

            # Go back 7 marbles
            node = current_node
            for _ in range(7):
                node = node.previous

            # Add to score
            score[current_elf] += node.data

            # Remove the node
            prev_node = node.previous
            next_node = node.next

            prev_node.next = next_node
            next_node.previous = prev_node

            current_node = next_node
        # Next elf
        current_elf = (current_elf + 1) % players

    return max(score.values())


# Part 1
part_1 = max_score(players, max_marble)
print(f'Part 1: {part_1}')

# Part 2
part_2 = max_score(players, max_marble * 100)
print(f'Part 2: {part_2}')

