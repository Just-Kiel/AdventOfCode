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
