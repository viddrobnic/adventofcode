from string import ascii_lowercase


class LinkedList:
    def __init__(self, value, successor=None):
        self.value = value
        self.successor = successor


def copy(l):
    """Copy linked list l"""
    if l is None:
        return None

    new_ll = LinkedList(l.value)
    i = new_ll
    j = l
    while j.successor is not None:
        i.successor = LinkedList(j.successor.value)
        i = i.successor
        j = j.successor

    return new_ll


def remove(p, options):
    """Remove items in options from linked listed p"""
    if p is None:
        return None

    lst = copy(p)
    prev = None
    i = lst
    while True:
        if i is None:
            break

        if i.value in options:
            if prev is None:
                lst = i.successor
            else:
                prev.successor = i.successor
        else:
            prev = i

        i = i.successor

    return lst


def length(p):
    """Calculate length of linked list"""
    count = 1
    i = p
    while i.successor is not None:
        count += 1
        i = i.successor

    return count


# Calculate polymer reaction
def react(p):
    polymer = copy(p)

    reacted = True
    while reacted:
        reacted = False

        prev = None
        i = polymer
        while True:
            if i is None or i.successor is None:
                break

            reaction = False
            if i.value.islower() and i.successor.value.isupper():
                reaction = i.value == i.successor.value.lower()
            elif i.value.isupper() and i.successor.value.islower():
                reaction = i.value == i.successor.value.upper()

            if reaction:
                reacted = True

                if prev is None:
                    polymer = i.successor.successor
                    i = polymer
                else:
                    prev.successor = i.successor.successor
                    i = prev.successor
            else:
                prev = i
                i = i.successor

    return polymer


# Read input
polymer_raw = input()
polymer = None
for c in reversed(polymer_raw):
    polymer = LinkedList(c, polymer)

# Part 1
res = react(polymer)
print(f'Part 1: {length(res)}')

# Part 2
minimum_count = None
for c in ascii_lowercase:
    print(f'Trying letter: {c}', end='\r')

    new_p = remove(polymer, (c, c.upper()))
    res = length(react(new_p))

    if minimum_count is None:
        minimum_count = res
    else:
        minimum_count = min(minimum_count, res)

print(f'\rPart 2: {minimum_count}         ')

