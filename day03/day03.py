import numpy
dayNumber = "03"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0
lengthNumber = 0
counted = False

for index, line in enumerate(Lines):
    line = line.strip()
    # print("Line {} : {}".format(index, line))

    for idChar, char in enumerate(line):
        if(char.isdigit()):
            lengthNumber += 1
        else:
            if(lengthNumber == 0):
                continue
            else:
                for i in range(index-1, index+2):
                    if(i < 0 or i >= len(Lines)):
                        continue
                    for j in range(idChar-lengthNumber-1, idChar+1):
                        if((i == index and j>idChar-lengthNumber-1 and j<idChar) or counted or j < 0 or j >= len(Lines[i])):
                            # print("Lines[i][j] : {} - i : {} - index : {} - j : {} - idChar : {} - lengthNumber : {}".format(Lines[i][j], i, index, j, idChar, lengthNumber))
                            continue
                        if(Lines[i][j] != "."):
                            # print(Lines[index][idChar])
                            print(line[idChar-lengthNumber:idChar])
                            print(lengthNumber)
                            print(index)
                            print(j)
                            result += int(line[idChar-lengthNumber:idChar])
                            counted = True
                            break

                lengthNumber = 0
                counted = False

    

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

print("Result : {}".format(result))