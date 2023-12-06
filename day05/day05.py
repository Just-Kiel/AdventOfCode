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

instructions = []
dictData = {}
for index, line in enumerate(Lines[2:]):
    index += 1
    line = line.strip()

    if line == "": 
        # append dict to instructions
        instructions.append(dictData)
        dictData = {}
        continue
    if line[0].isalpha(): continue

    info = [int(x) for x in line.split(" ")]

    keys = range(info[1], info[1]+info[2])
    values = range(info[0], info[0]+info[2])
    dictData.update(dict(zip(keys, values)))
    # id = 0
    # for key in range(info[1], info[1]+info[2]):
    #     dictData[key] = info[0]+id
        # id+=1

instructions.append(dictData)

for instruction in instructions:
    for index, seed in enumerate(seeds):
        seeds[index] = instruction[seed] if seed in instruction else seed

result = min(seeds)
print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

print("Result : {}".format(result))