#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

int main()
{
    // PART ONE - DAY ONE
    std::string str = "";
    std::fstream fin("../../../day1/src/elvesCalories.txt", std::fstream::in);

    unsigned int elvesNb = 0;
    std::vector<int> sumCaloriesElves = {0};
    while(!fin.eof())
    {
        std::getline(fin, str, '\n');

        if (str.size() == 0)
        {
            // create instance for new elf
            elvesNb++;
            sumCaloriesElves.push_back(0);
        } else {
            // make the sum of his calories
            sumCaloriesElves[elvesNb] += std::stoi(str);
        }
    }

    // get the max
    std::vector<int>::iterator result = std::max_element(sumCaloriesElves.begin(), sumCaloriesElves.end());

    std::cout << std::distance(sumCaloriesElves.begin(), result) + 1 << " with " << *result << std::endl;


    // PART TWO - DAY ONE
    int sumTopThree = *result;

    for (size_t i = 0; i < 2; i++)
    {
        sumCaloriesElves.erase(sumCaloriesElves.begin() + std::distance(sumCaloriesElves.begin(), result));
        // get the max
        result = std::max_element(sumCaloriesElves.begin(), sumCaloriesElves.end());

        std::cout << std::distance(sumCaloriesElves.begin(), result) + 1 << " with " << *result << std::endl;
    
        sumTopThree += *result;
    }

    std::cout << sumTopThree << std::endl;
    // expected result : 207410
    return 0;
}