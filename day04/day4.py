print ("Day Four - Part One !")
fichier = open("day04/input.txt", "r")

Lines = fichier.readlines()

def convertToPair(line):
    return line.split(',')

def convertToElf(pair):
    elf = pair.split('-')
    elf = [int(step) for step in elf]
    elf = [i for i in range(elf[0], elf[1]+1)]
    return elf

count = 0
for line in Lines:
    # print(line)

    pair = convertToPair(line[:-1])
    # print(pair)

    elves = [convertToElf(elf) for elf in pair]
    # print(elves)

    commonAreas = set(elves[0]).intersection(elves[1])

    sizeCommon = len(commonAreas)

    if sizeCommon == len(elves[0]) or sizeCommon == len(elves[1]):
        count += 1

# expected : 526
print("Total Common Areas : {}".format(count))

print ("Day Four - Part Two !")

countOverlaps = 0
for line in Lines:
    # print(line)

    pair = convertToPair(line[:-1])
    # print(pair)

    elves = [convertToElf(elf) for elf in pair]
    # print(elves)

    commonAreas = set(elves[0]).intersection(elves[1])

    sizeCommon = len(commonAreas)

    if sizeCommon != 0:
        countOverlaps += 1

# expected : 886
print("Total Overlaps : {}".format(countOverlaps))