#include <iostream>
#include <string>
#include <vector>

int main() {
    int s_1 = 0, s_2 = 0;

    std::string line;
    while (std::getline(std::cin, line)) {
        std::vector<int> numbers;
        std::string buffer = "";
        for (int i = 0; i < line.size(); ++i) {
            if (line[i] == ' ' || line[i] == '\t') {
                if (buffer == "") continue;
                numbers.push_back(std::stoi(buffer));
                buffer = "";
            } else buffer += line[i];
        }
        if (buffer != "") numbers.push_back(std::stoi(buffer));

        int min = numbers[0], max = numbers[0];
        for (int i = 0; i < numbers.size() - 1; ++i) {
            for (int j = i + 1; j < numbers.size(); ++j) {
                if (numbers[i] < min) min = numbers[i];
                if (numbers[i] > max) max = numbers[i];

                if (numbers[i] % numbers[j] == 0) s_2 += numbers[i] / numbers[j];
                if (numbers[j] % numbers[i] == 0) s_2 += numbers[j] / numbers[i];
            }
        }
        if (numbers.back() < min) min = numbers.back();
        if (numbers.back() > max) max = numbers.back();
        s_1 += max - min;
    }

    std::cout << "Part One: " << s_1 << std::endl << "Part Two: " << s_2 << std::endl;

    return 0;
}
