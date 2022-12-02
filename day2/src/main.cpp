#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>

int main()
{
    // PART ONE - DAY TWO
    std::string str = "";
    std::fstream fin("../../../day2/src/strategyGuide.txt", std::fstream::in);

    int totalScore = 0;
    int choiceScore = 0;
    int matchScore = 0;

    while(!fin.eof())
    {
        int tempScore = 0;
        std::getline(fin, str, '\n');

        char opponentChoice = str[0];
        char myChoice = str[2];

        switch (myChoice)
        {
        case 'X':
            tempScore = 1;
            choiceScore += 1;
            break;
            
        case 'Y':
            tempScore = 2;
            choiceScore += 2;
            break;
        
        case 'Z':
            tempScore = 3;
            choiceScore += 3;
            break;
        
        default:
            break;
        }

        switch (opponentChoice)
        {
            // Rock
        case 'A':
            if(tempScore == 1) matchScore += 3;
            if(tempScore == 2) matchScore += 6;
            if(tempScore == 3) matchScore += 0;
            break;
            
            // Paper
        case 'B':
            if(tempScore == 1) matchScore += 0;
            if(tempScore == 2) matchScore += 3;
            if(tempScore == 3) matchScore += 6;
            break;
            
            // Scissors
        case 'C':
            if(tempScore == 1) matchScore += 6;
            if(tempScore == 2) matchScore += 0;
            if(tempScore == 3) matchScore += 3;
            break;
        
        default:
            break;
        }
    }

    totalScore = matchScore + choiceScore;

    std::cout << totalScore << std::endl;

    return 0;
}