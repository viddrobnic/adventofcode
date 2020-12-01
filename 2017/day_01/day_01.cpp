#include <iostream>

int main() {
    std::string input;
    std::cin >> input;

    int s_1 = 0, s_2 = 0;

    for (int i = 0; i < input.size(); ++i) {
        int number = input[i] - '0';
        int next_1 = input[(i + 1) % input.size()] - '0';
        int next_2 = input[(i + input.size() / 2) % input.size()] - '0';

        if (number == next_1) s_1 += number;
        if (number == next_2) s_2 += number;
    }

    std::cout << "Part One: " << s_1 << "\nPart Two: " << s_2 << std::endl;

    return 0;
}
