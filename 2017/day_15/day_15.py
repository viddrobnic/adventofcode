start_a = int(input().split()[-1])
start_b = int(input().split()[-1])

mask = 2**16 - 1  # Mask for last 16 bits

gen_a = start_a
gen_b = start_b

count_1 = 0  # Counter for part one
for i in range(40000000):
    gen_a = (gen_a * 16807) % 2147483647   # Generate A
    gen_b = (gen_b * 48271) % 2147483647  # Generate B

    bin_a = gen_a & mask  # Last 16 bits of A
    bin_b = gen_b & mask  # Last 16 bits of B
    if bin_a == bin_b:
        count_1 += 1

# Reset generators
gen_a = start_a
gen_b = start_b

count_2 = 0  # Counter for part two
for i in range(5000000):
    while True:
        gen_a = (gen_a * 16807) % 2147483647
        if gen_a % 4 == 0:  # Take only the numbers devisible by 4
            break

    while True:
        gen_b = (gen_b * 48271) % 2147483647
        if gen_b % 8 == 0:  # Take only the numbers devisible by 8
            break

    bin_a = gen_a & mask  # Last 16 bits of A
    bin_b = gen_b & mask  # Last 16 bits of B
    if bin_a == bin_b:
        count_2 += 1

print('Part One: {}\nPart Two: {}'.format(count_1, count_2))
