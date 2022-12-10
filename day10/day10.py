from dataclasses import dataclass
print ("Day Ten - Part One !")
fichier = open("day10/input.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]

lines = [x for x in [line.split(' ') for line in lines]]

def valueAt(stop):
    X = 1
    currentLine = 0
    addCountCycle=0
    # v2
    for i in range(stop-1):
        if lines[currentLine][0] == "noop":
            currentLine+=1
        elif addCountCycle == 1:
            X+=int(lines[currentLine][1])
            addCountCycle=0
            currentLine+=1
        elif addCountCycle == 0 :
            addCountCycle=1
    return X

valuesToStop = [x*40+20 for x in range(0, 6)]

result = sum([valueAt(x)*x for x in valuesToStop])

print(f"Sum of 6 signals : {result}")
# expected : 15360

# ---------------------------------
print ("Day Ten - Part Two !")

# view = [[] for x in range(6)]
# view[0] = "#"

view = ["#", *["" for x in range(5)]]


X=1
sprite = [X-1, X, X+1]

stop = 241
currentView = 0
currentLine = 0
addCountCycle=0
# v2
for i in range(stop-1):
    if currentView == 6: break
    if lines[currentLine][0] == "noop":
        currentLine+=1
    elif addCountCycle == 1:
        X+=int(lines[currentLine][1])
        addCountCycle=0
        currentLine+=1
    elif addCountCycle == 0 :
        addCountCycle=1

    sprite = [X-1, X, X+1]
    # print(len(view[currentView]))
    if len(view[currentView]) in sprite:
        view[currentView] += "#"
    else :
        view[currentView] += " "

    if len(view[currentView]) == 40:
        currentView+=1

[print(view[i]) for i in range(6)]
# expected : PHLHJGZA