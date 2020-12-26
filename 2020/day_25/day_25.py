card_public_key, door_public_key = 9033205, 9281649
subject_number = 7
modulus = 20201227


def part_one():
    value = 1

    loop_size = 1
    while True:
        value = (value * subject_number) % modulus

        if value == card_public_key:
            break

        loop_size += 1

    return pow(door_public_key, loop_size, modulus)


if __name__ == '__main__':
    print(f'Part One: {part_one()}')
