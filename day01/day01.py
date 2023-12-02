print ("Day One - Part one !")
fichier = open("day01/input.txt", "r")

Lines = fichier.readlines()
result = 0
for line in Lines:
    allDigits :list[int] = []

    for char in line:
        if (char.isdigit()):
            allDigits.append(int(char))

    if len(allDigits) != 0:
        digitsToAdd = [allDigits[0], allDigits[-1]] if len(allDigits) >1 else [allDigits[0], allDigits[0]]

        digitsToAdd = ''.join(map(str, digitsToAdd))

        result += int(digitsToAdd)

print("Result : {}".format(result))

print ("Day One - Part two !")

# TODO : Not working for now but I'm too tired to fix it

result = 0
for line in Lines:
    allDigits :list[int] = []
    stringToParse = ""

    for i, char in enumerate(line):
        if (char.isdigit()):
            allDigits.append(int(char))

        # From https://www.youtube.com/watch?v=rnidYOt9m2o but don't understand the difference because mine not working
        for d, val in enumerate(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]):
            if line[i:].startswith(val):
                allDigits.append(d+1)
                
        # else :
        #     stringToParse += char
        #     # Check if string to parse contains something
        #     if (stringToParse != ""):
        #         sizeBefore = len(allDigits)
        #         if "one" in stringToParse:
        #             allDigits.append(1)
        #         elif "two" in stringToParse:
        #             allDigits.append(2)
        #         elif "three" in stringToParse:
        #             allDigits.append(3)
        #         elif "four" in stringToParse:
        #             allDigits.append(4)
        #         elif "five" in stringToParse:
        #             allDigits.append(5)
        #         elif "six" in stringToParse:
        #             allDigits.append(6)
        #         elif "seven" in stringToParse:
        #             allDigits.append(7)
        #         elif "eight" in stringToParse:
        #             allDigits.append(8)
        #         elif "nine" in stringToParse:
        #             allDigits.append(9)
        #         elif "zero" in stringToParse:
        #             allDigits.append(0)
                
        #     if sizeBefore != len(allDigits) :stringToParse = ""

    digitsToAdd = [allDigits[0], allDigits[-1]] if len(allDigits) >1 else [allDigits[0], allDigits[0]]

    digitsToAdd = ''.join(map(str, digitsToAdd))

    result += int(digitsToAdd)
print("Result : {}".format(result))
# 55648 too low