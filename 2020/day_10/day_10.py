from collections import defaultdict

with open('in') as f:
    data = list(map(lambda x: int(x.strip()), f.readlines()))
    data.append(0)
    data.append(max(data) + 3)
    data.sort()


def part_one():
    diffs = defaultdict(int)

    for i in range(len(data) - 1):
        diffs[data[i + 1] - data[i]] += 1

    return diffs[1] * diffs[3]


def part_two():
    memo = dict()

    def solver(start_index):
        if start_index == len(data) - 1:
            return 1

        if start_index in memo:
            return memo[start_index]

        count = 0
        for i in range(start_index + 1, len(data)):
            if data[i] - data[start_index] <= 3:
                count += solver(i)
            else:
                break

        memo[start_index] = count
        return count

    return solver(0)


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
    print(f'Part Two: {part_two()}')
