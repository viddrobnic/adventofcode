from collections import deque
from copy import deepcopy

deck_1 = deque()
deck_2 = deque()

with open('in') as f:
    player_one = True
    for line in f.readlines():
        line = line.strip()
        if line == '':
            player_one = False
            continue

        try:
            card = int(line)
            if player_one:
                deck_1.append(card)
            else:
                deck_2.append(card)
        except:
            pass


def part_one(deck_1, deck_2):
    while len(deck_1) > 0 and len(deck_2) > 0:
        card1 = deck_1.popleft()
        card2 = deck_2.popleft()

        if card1 > card2:
            deck_1.append(card1)
            deck_1.append(card2)
        else:
            deck_2.append(card2)
            deck_2.append(card1)

    if len(deck_1) > 0:
        deck = deck_1
    else:
        deck = deck_2

    res = 0
    multiplier = 1
    while deck:
        res += multiplier * deck.pop()
        multiplier += 1

    return res


def part_two(deck_1, deck_2):
    def game(deck_1, deck_2):
        player1_memo = set()
        player2_memo = set()

        while len(deck_1) > 0 and len(deck_2) > 0:
            deck_1_key = ','.join(map(str, deck_1))
            deck_2_key = ','.join(map(str, deck_2))

            if deck_1_key in player1_memo:
                return 1, deck_1

            if deck_2_key in player2_memo:
                return 1, deck_1

            player1_memo.add(deck_1_key)
            player2_memo.add(deck_2_key)

            card1 = deck_1.popleft()
            card2 = deck_2.popleft()

            if len(deck_1) >= card1 and len(deck_2) >= card2:
                new_deck_1 = deque()
                new_deck_2 = deque()

                for i in range(card1):
                    new_deck_1.append(deck_1[i])

                for i in range(card2):
                    new_deck_2.append(deck_2[i])

                winner, _ = game(new_deck_1, new_deck_2)
                if winner == 1:
                    deck_1.append(card1)
                    deck_1.append(card2)
                else:
                    deck_2.append(card2)
                    deck_2.append(card1)
            else:
                if card1 > card2:
                    deck_1.append(card1)
                    deck_1.append(card2)
                else:
                    deck_2.append(card2)
                    deck_2.append(card1)

        if len(deck_1) > 0:
            return 1, deck_1
        else:
            return 2, deck_2

    _, deck = game(deck_1, deck_2)
    res = 0
    multiplier = 1
    while deck:
        res += multiplier * deck.pop()
        multiplier += 1

    return res


if __name__ == '__main__':
    print(f'Part One: {part_one(deepcopy(deck_1), deepcopy(deck_2))}')
    print(f'Part Two: {part_two(deepcopy(deck_1), deepcopy(deck_2))}')