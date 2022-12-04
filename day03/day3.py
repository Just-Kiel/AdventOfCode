print ("Day Three - Part one !")
fichier = open("day03/input.txt", "r")

Lines = fichier.readlines()

def convertInInt(char):
    return ord(char) - 38 - 58*char.islower()

    # Version 1 less great
    # if char.islower():
    #     result = ord(char)-(ord('a')-1)
    # else:
    #     result = ord(char)-65+27
    # return result

totalError = 0
i = 0
for line in Lines:
    charAsInt: list[int] = list(map(convertInInt, line[:-1]))
    halfSize= int(len(charAsInt)/2)
    # print(charAsInt)

    occurence = set(charAsInt[:halfSize]).intersection(charAsInt[halfSize:])
    totalError += list(occurence)[0]
    
# expected : 7850
print("Total error : {}".format(totalError))

print ("Day Three - Part Two !")
groups = [Lines[x:x+3] for x in range(0, len(Lines), 3)]

totalPriority = 0
for group in groups:
    lines = [group[x:x+1][0] for x in range(0, len(group), 1)]

    # groupAsInt = [list(map(oui, line[:-1])) for line in lines]

    groupAsInt = [[convertInInt(char) for char in line[:-1]] for line in lines]

    commonItem = set(groupAsInt[0]).intersection(groupAsInt[1]).intersection(groupAsInt[2])
    # print(commonItem)

    totalPriority += list(commonItem)[0]

# expected : 2581
print("Total priority : {}".format(totalPriority)) 