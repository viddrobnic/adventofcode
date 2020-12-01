raw_data = input()
lengths = list(map(int, raw_data.split(',')))  # Get data for first part
skip_size = 0
current_position = 0
size = 256  # Size of list for debug purposes

data = [i for i in range(size)]  # Create array of numbers for first part

for i in lengths:
    buff = []  # Sublist to be reversed
    for j in range(current_position, current_position + i):  # Fill the buffer
        buff.append(data[j % size])

    buff = buff[::-1]  # Reverse the buffer

    for j in range(len(buff)):  # Store the buffer back to data
        data[(j + current_position) % size] = buff[j]

    current_position += i + skip_size  # Increase current position
    current_position %= size  # Wrap it around
    skip_size += 1  # Increase skip size

answer_1 = data[0] * data[1]  # Store answer for first part

data = [i for i in range(size)]  # Clean array for part two
skip_size = 0  # Reset skip size
current_position = 0  # Reset current position

lengths = list(map(ord, raw_data))  # Get data for second part
lengths += [17, 31, 73, 47, 23]

for _ in range(64):
    for i in lengths:  # Same algorithm as before
        buff = []
        for j in range(current_position, current_position + i):
            buff.append(data[j % size])

        buff = buff[::-1]

        for j in range(len(buff)):
            data[(j + current_position) % size] = buff[j]

        current_position += i + skip_size
        current_position %= size
        skip_size += 1

d_hash = ''  # Final dense hash
for i in range(16):  # Go through blocks
    block = 0
    for j in range(i * 16, i* 16 + 16):  # Go through numbers in block
        block ^= data[j]  # XOR current number with next number in block
    d_hash += hex(block)[2:].zfill(2)  # Append hex value of block number without '0x' at the beginning

print('Part One: {}\nPart Two: {}'.format(answer_1, d_hash))
