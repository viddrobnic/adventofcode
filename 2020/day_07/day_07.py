import re
from collections import defaultdict, deque

graph = defaultdict(set)

p = re.compile(r'(\d*) ?([a-z ]+) bag')


def clean_bag(b):
    match = p.search(b)
    d = match.group(1)

    if d == '':
        d = 0
    else:
        d = int(d)

    return d, match.group(2)


with open('in') as f:
    for line in f:
        start, other = line.strip().split('contain')

        start = p.search(start).group(2)
        other = list(map(clean_bag, other.split(',')))

        graph[start].update(other)


def part_one():
    goal = 'shiny gold'
    contains = dict()

    def dfs(v):
        if v == goal:
            return True

        if v not in graph:
            return False

        if v in contains:
            return contains[v]

        contains[v] = False
        for _, u in graph[v]:
            contains[v] = dfs(u) or contains[v]

        return contains[v]

    for i in graph.keys():
        dfs(i)

    count = 0
    for k in contains.keys():
        if contains[k]:
            count += 1

    return count


def part_two():
    memo = dict()

    def dfs(v):
        if v in memo:
            return memo[v]

        if v not in graph:
            return 0

        memo[v] = 0
        for d, u in graph[v]:
            memo[v] += d + d * dfs(u)

        return memo[v]

    return dfs('shiny gold')


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
