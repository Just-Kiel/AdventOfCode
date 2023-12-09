import numpy

dayNumber = "08"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0

sequence = Lines[0].strip().replace("L", "0").replace("R", "1")

# Pseudo code
# while instruction != "ZZZ"
#  take sequence and apply it to the map

maps = [[line.strip().split(" = ")[0], line.strip().split(" = ")[1].replace("(", "").replace(")", "").split(", ")] for line in Lines[2:]]

currentPoint = "AAA"
while currentPoint != "ZZZ":
    for map in maps:
        if map[0] == currentPoint:
            currentPoint = map[1][int(sequence[result%len(sequence)])]
            result += 1
            break 

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

currentPoints = [maps[x][0] for x in range(len(maps)) if maps[x][0].endswith("A")]
print(currentPoints)

countZpoints = 0
while countZpoints < len(currentPoints):
    countZpoints = 0
    for i in range(len(currentPoints)):
        index = [x for x in range(len(maps)) if maps[x][0] == currentPoints[i]]
        currentPoints[i] = maps[index[0]][1][int(sequence[result%len(sequence)])]
        if currentPoints[i].endswith("Z"):
            countZpoints += 1

    result += 1
    # print(countZpoints)

print("Result : {}".format(result))