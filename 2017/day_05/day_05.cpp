#include <iostream>
#include <vector>
#include <string>

int main() {
    std::vector<int> data;

    std::string line;
    while(std::getline(std::cin, line)) data.push_back(std::stoi(line));
    std::vector<int> data_2 = data;


    int steps_1 = 0;
    int i = 0;
    while (i < data.size()) {
        int a = data[i];
        ++data[i];
        i += a;
        ++steps_1;
    }

    int steps_2 = 0;
    i = 0;
    while(i < data_2.size()) {
        int a = data_2[i];
        if (a >= 3) --data_2[i];
        else ++data_2[i];
        i += a;
        ++steps_2;
    }

    std::cout << "Part One: " << steps_1 << std::endl << "Part Two: " << steps_2 << std::endl;

    return 0;
}
