#include <iostream>
#include <bitset>

int main() {
    long long start_a = 591;  // Your input (not readeing from cin because of ugly data)
    long long start_b = 393;

    int count_1 = 0, count_2 = 0;  // Set counters for part one and two

    long long gen_a = start_a;
    long long gen_b = start_b;
    for (int i = 0; i < 40000000; ++i) {
        gen_a = (gen_a * 16807) % 2147483647;  // Generate A
        gen_b = (gen_b * 48271) % 2147483647;  // Generate B

        if (std::bitset<16>(gen_a) == std::bitset<16>(gen_b)) ++count_1;  // Check if last 16 bits match
    }

    // Reset generators
    gen_a = start_a;
    gen_b = start_b;
    for (int i = 0; i < 5000000; ++i) {
        do {
            gen_a = (gen_a * 16807) % 2147483647;
        } while (gen_a % 4 != 0);  // Only take numbers devisible by 4

        do {
            gen_b = (gen_b * 48271) % 2147483647;
        } while (gen_b % 8 != 0); // Only take numbers devisible by 8

        if (std::bitset<16>(gen_a) == std::bitset<16>(gen_b)) ++count_2;  // Check if last 16 bits match
    }

    std::cout << "Part One: " << count_1 << std::endl << "Part Two: " << count_2 << std::endl;

    return 0;
}
