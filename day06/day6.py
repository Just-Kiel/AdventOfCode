print ("Day Six - Part One !")

fichier = open("day06/input.txt", "r")
Lines = fichier.readlines()

# Lines = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
# goal : verify with test : here 7

def compareNewCharacter(str, char):
    return [char in str, str.find(char)]

def day6Function(size):
    startMessage = Lines[0]
    resultMessage = 0
    for i in range(1, sizeString):
        # compare with potential new character
        check = compareNewCharacter(startMessage, Lines[i])
        startMessage += Lines[i]

        # get index where occurs
        startMessage = startMessage[(check[1]+1) if check[0] else 0:]

        sizeMessage = len(startMessage)

        if(sizeMessage == size): 
            resultMessage=i 
            break
    return [resultMessage, startMessage]


# needed bc take 2 lines
Lines=Lines[0]

sizeString = len(Lines)
print(sizeString)

part1 = day6Function(4)

print(f"Start of buffer : {part1[0]+1} with {part1[1]}")
# expected : 1658 with sblq


print ("Day Six - Part Two !")

part2 = day6Function(14)

print(f"Start of message : {part2[0]+1} with {part2[1]}")
# expected : 2260 with ctlwqhfnbmgspr