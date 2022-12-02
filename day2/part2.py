print ("Day Two - Part two !")
fichier = open("day2/src/strategyGuide.txt", "r")

Lines = fichier.readlines()

matchScore = 0
# Strips the newline character
for line in Lines:
    tempScore = 0
    myChoice = line[2]
    opponentChoice = line[0]
    # print(line[0]+line[2])
    if myChoice == 'X':
        tempScore = 0
    elif myChoice == 'Y':
        tempScore = 3
    elif myChoice == 'Z':
        tempScore = 6

    matchScore += tempScore

    # Rock
    if opponentChoice=='A':
        if tempScore == 0:
            matchScore += 3 #Scissors
        elif tempScore == 3:
            matchScore += 1 #Rock
        elif tempScore == 6:
            matchScore += 2 #Paper

    # Paper
    elif opponentChoice=='B':
        if tempScore == 6:
            matchScore += 3 #Scissors
        elif tempScore == 0:
            matchScore += 1 #Rock
        elif tempScore == 3:
            matchScore += 2 #Paper
            
    # Scissors
    elif opponentChoice=='C':
        if tempScore == 3:
            matchScore += 3 #Scissors
        elif tempScore == 6:
            matchScore += 1 #Rock
        elif tempScore == 0:
            matchScore += 2 #Paper

print("Total Score is : {}".format(matchScore))
