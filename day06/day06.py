dayNumber = "06"
print ("Day {} - Part one !".format(dayNumber))
fichier = open("day{}/input.txt".format(dayNumber), "r")

Lines = fichier.readlines()
result = 1
time = [int(x) for x in Lines[0].strip().split(": ")[1].split(" ") if x != ""]
distance = [int(x) for x in Lines[1].strip().split(": ")[1].split(" ") if x != ""]

for id, t in enumerate(time):
    count = 0
    for index in range(t):
        maxDistance = index * (t-index)
        if maxDistance > distance[id]: 
            # print("maxDistance : {}".format(maxDistance))
            count+=1

    result *= count

print("Result : {}".format(result))

print ("Day {} - Part two !".format(dayNumber))
result = 0

time = [str(x) for x in time]
distance = [str(x) for x in distance]
time = int(''.join(time))
distance = int(''.join(distance))

for index in range(time):
    maxDistance = index * (time-index)
    if maxDistance > distance: 
        result+=1

print("Result : {}".format(result))