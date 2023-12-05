import numpy
dayNumber = "04"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 0

for index, line in enumerate(Lines):
    line = line.strip()
    content = line.split(": ")
    numbers = []
    numbers = content[1].split(" | ")
    numbers = [[int(x) for x in number.split(" ") if x!=""] for number in numbers]

    numbers = [x for x in numbers[0] if x in numbers[1]]

    if len(numbers)>0 : result += 2**(len(numbers)-1)

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0
instanceOfCard = [1 for x in Lines]

for index, line in enumerate(Lines):
    line = line.strip()
    content = line.split(": ")
    numbers = []
    numbers = content[1].split(" | ")
    numbers = [[int(x) for x in number.split(" ") if x!=""] for number in numbers]

    numbers = [x for x in numbers[0] if x in numbers[1]]
    
    for instance in range(instanceOfCard[index]):
        for copy in range(len(numbers)):
            instanceOfCard[index+copy+1] +=1

result = sum(instanceOfCard)

print("Result : {}".format(result))