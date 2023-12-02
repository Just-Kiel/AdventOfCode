import numpy
dayNumber = "02"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0

for line in Lines:
    begin = line.split(": ")
    nbGame = int(begin[0].split("Game ")[1])
    line = begin[1]
    gameWorking = True

    line = line.split("; ")

    for set in line:
        set = set.split(", ")
        for color in set:
            number = int(color.split(" ")[0])
            color = color.split(" ")[1]

            if (("red" in color and number > 12) or ("green" in color and number > 13) or ("blue" in color and number > 14)) :
                # print("Game {} impossible ".format(nbGame))
                gameWorking = False

    if gameWorking==True :
        # print("Add {}".format(nbGame))
        result+=nbGame 

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

for line in Lines:
    begin = line.split(": ")
    nbGame = int(begin[0].split("Game ")[1])
    line = begin[1]
    gameWorking = True
    maxRGB = [0, 0, 0]

    line = line.split("; ")

    for set in line:
        set = set.split(", ")
        for color in set:
            number = int(color.split(" ")[0])
            color = color.split(" ")[1]

            if ("red" in color and number > maxRGB[0]):
                maxRGB[0] = number
            elif ("green" in color and number > maxRGB[1]):
                maxRGB[1] = number
            elif ("blue" in color and number > maxRGB[2]):
                maxRGB[2] = number
        
    result += numpy.prod(maxRGB)

print("Result : {}".format(result))