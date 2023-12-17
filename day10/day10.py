import numpy

dayNumber = "10"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0

map = [line.strip() for line in Lines]
# print(map)

waypoint = ""
lineCurrent = 0
charCurrent = -1
while waypoint != "S":
    charCurrent += 1
    if charCurrent >= len(map[lineCurrent]):
        lineCurrent += 1
        charCurrent = -1
    waypoint = map[lineCurrent][charCurrent]

startPoint = [lineCurrent, charCurrent]

print("Waypoint : {}, Index : {}".format(waypoint, startPoint))
tempPoint = ""
comingFrom = [-1, -1]
while tempPoint != "S":
    comingFrom = [lineCurrent, charCurrent]
    print(tempPoint, comingFrom)
    if map[lineCurrent+1][charCurrent] == "|" and comingFrom != [lineCurrent+1, charCurrent]:
        print('down')
        lineCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent-1][charCurrent] == "|" and comingFrom != [lineCurrent-1, charCurrent]:
        print('up')
        lineCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue

    if map[lineCurrent][charCurrent+1] == "-" and comingFrom != [lineCurrent, charCurrent+1]:
        print('right')
        charCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent][charCurrent-1] == "-" and comingFrom != [lineCurrent, charCurrent-1]:
        print('left')
        charCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue

    if map[lineCurrent+1][charCurrent] == "L" and comingFrom != [lineCurrent+1, charCurrent+1]:
        print('down right')
        lineCurrent +=1
        charCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent][charCurrent-1] == "L" and comingFrom != [lineCurrent-1, charCurrent-1]:
        print('left up')
        lineCurrent -=1
        charCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue

    if map[lineCurrent][charCurrent+1] == "7" and comingFrom != [lineCurrent+1, charCurrent+1]:
        print('right down')
        lineCurrent +=1
        charCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent-1][charCurrent] == "7" and comingFrom != [lineCurrent-1, charCurrent-1]:
        print('up left')
        lineCurrent -=1
        charCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    
    if map[lineCurrent+1][charCurrent] == "J" and comingFrom != [lineCurrent+1, charCurrent-1]:
        print('down left')
        lineCurrent +=1
        charCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent][charCurrent+1] == "J" and comingFrom != [lineCurrent-1, charCurrent+1]:
        print('right up')
        lineCurrent -=1
        charCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    
    if map[lineCurrent-1][charCurrent] == "F" and comingFrom != [lineCurrent-1, charCurrent+1]:
        print('up right')
        lineCurrent -=1
        charCurrent +=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    if map[lineCurrent][charCurrent-1] == "F" and comingFrom != [lineCurrent+1, charCurrent-1]:
        print('left down')
        lineCurrent +=1
        charCurrent -=1
        tempPoint = map[lineCurrent][charCurrent]
        continue
    
print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

print("Result : {}".format(result))