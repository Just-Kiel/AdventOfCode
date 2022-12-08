print ("Day Height - Part One !")

# look here to check the indexes around : https://www.youtube.com/watch?v=AwWOMaMejwU
fichier = open("day08/input.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]

sizeLine = len(lines[0])

#to enable on real input
lines = lines[:-1]
nbLines = len(lines)
total = sizeLine*2 +(nbLines-2)*2

print(f"Total visible around : {total}")

columns = []
for i in range(0, sizeLine):
    column = []
    for j in range(0, nbLines):
        column.append(lines[j][i])
    column=''.join(column)
    columns.append(column)

for index in range(1, nbLines-1):
    for i in range(1, sizeLine-1):
        if index+1 < nbLines-1: MaxColumnBottom = max(columns[i][index+1:])
        else : MaxColumnBottom = columns[i][nbLines-1]
        if index >1: MaxColumnTop = max(columns[i][:index])
        else : MaxColumnTop = columns[i][0]
        if i >1: MaxLineLeft = max(lines[index][:i])
        else : MaxLineLeft = lines[index][0]
        if i+1 <= sizeLine-1 : MaxLineRight = max(lines[index][i+1:])
        else: MaxLineRight = lines[index][sizeLine-1]

        if lines[index][i] > MaxColumnBottom or lines[index][i] > MaxColumnTop or lines[index][i] > MaxLineLeft or lines[index][i] > MaxLineRight:
            total+=1
            

print(f"Total visible : {total}")
# expected : 1719