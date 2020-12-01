data = input()
score = 0  # Score of all groups
chars = 0  # Number of non-canceled chracter within the garbage
garbage = False  # Is currently in garbage
depth = 0  # Current depth
ignore = False  # Should ignore current character

for c in data:  # Check each character in data
    if ignore:  # If should ignore current character do so
        ignore = False
        continue

    if garbage:  # Handle if in garbage
        if c == '!':  # Handle ignore
            ignore = True
        elif c == '>':  # Handle end garbage
            garbage = False
        else:  # Add to character in garbage
            chars += 1
        continue

    if c == '{':  # Handle beginning of group
        depth += 1

    if c == '<':  # Handle beginning of garbage
        garbage = True

    if c == '}':  # Handle ending of group
        score += depth
        depth -= 1

print('Part One: {}\nPart Two: {}'.format(score, chars))
