dayNumber = "05"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0
seeds = [int(x) for x in Lines[0].split(": ")[1].split(" ")]

# Pseudo code
# First pass
# Get information about maps and compose a dict

# Second pass
# Just pass seeds and get the result

# Version 2
newInstruction = True
for index, seed in enumerate(seeds):
    newInstruction = True
    for line in Lines[2:]:
        line = line.strip()
        if line == "": 
            newInstruction = True
            continue
        if line[0].isalpha(): continue
        
        if newInstruction:
            info = [int(x) for x in line.split(" ")]
            
            if seeds[index] >= info[1] and seeds[index] < info[1]+info[2]:
                seeds[index] = info[0]+(seeds[index]-info[1])
                newInstruction = False

result = min(seeds)
print("Result : {}".format(result))
# 1181555926

print ("Day {} - Part two !".format(dayNumber))
result = 0

seeds = []
infoLine = [int(x) for x in Lines[0].split(": ")[1].split(" ")]
index = 0
while index < len(infoLine):
    seeds.extend(range(infoLine[index], infoLine[index]+infoLine[index+1]+1))
    index += 2
    
newInstruction = True
for index, seed in enumerate(seeds):
    newInstruction = True
    for line in Lines[2:]:
        line = line.strip()
        if line == "": 
            newInstruction = True
            continue
        if line[0].isalpha(): continue
        
        if newInstruction:
            info = [int(x) for x in line.split(" ")]
            
            if seeds[index] >= info[1] and seeds[index] < info[1]+info[2]:
                seeds[index] = info[0]+(seeds[index]-info[1])
                newInstruction = False

result = min(seeds)
print("Result : {}".format(result))