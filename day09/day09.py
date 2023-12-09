import numpy

dayNumber = "09"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0

histories = [[int(x) for x in line.strip().split(" ")] for line in Lines]

for history in histories:
    steps = [history.copy()]
    index = 0
    while steps[index].count(0) != len(steps[index]):
        steps.append([steps[index][x+1]-steps[index][x] for x in range(len(steps[index])-1)])
        index+=1
    
    steps.sort(key=lambda step:len(step))
    steps[0].append(0)

    for x in range(1, len(steps)):
        steps[x].append(steps[x-1][-1]+steps[x][-1])
    result+=steps[-1][-1]
        

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

for history in histories:
    steps = [history.copy()]
    index = 0
    while steps[index].count(0) != len(steps[index]):
        steps.append([steps[index][x+1]-steps[index][x] for x in range(len(steps[index])-1)])
        index+=1
    
    steps.sort(key=lambda step:len(step))
    steps[0].insert(0, 0)

    for x in range(1, len(steps)):
        steps[x].insert(0, steps[x][0]-steps[x-1][0])
    result+=steps[-1][0]

print("Result : {}".format(result))