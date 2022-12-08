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
for col in range(0, sizeLine):
    column = []
    for j in range(0, nbLines):
        column.append(lines[j][col])
    column=''.join(column)
    columns.append(column)

for row in range(1, nbLines-1):
    for col in range(1, sizeLine-1):
        if row+1 < nbLines-1: MaxColumnBottom = max(columns[col][row+1:])
        else : MaxColumnBottom = columns[col][nbLines-1]
        if row >1: MaxColumnTop = max(columns[col][:row])
        else : MaxColumnTop = columns[col][0]
        if col >1: MaxLineLeft = max(lines[row][:col])
        else : MaxLineLeft = lines[row][0]
        if col+1 <= sizeLine-1 : MaxLineRight = max(lines[row][col+1:])
        else: MaxLineRight = lines[row][sizeLine-1]

        if lines[row][col] > MaxColumnBottom or lines[row][col] > MaxColumnTop or lines[row][col] > MaxLineLeft or lines[row][col] > MaxLineRight:
            total+=1
            

print(f"Total visible : {total}")
# expected : 1719

# ---------------------------------
print ("Day Height - Part Two !")

scores = []
# distances on edges and corners don't matter because *0 at least on one side so = 0
for row in range(1, nbLines-1):
    for col in range(1, sizeLine-1):
        if row+1 < nbLines-1: MaxColumnBottom = max(columns[col][row+1:])
        else : MaxColumnBottom = columns[col][nbLines-1]
        if row >1: MaxColumnTop = max(columns[col][:row])
        else : MaxColumnTop = columns[col][0]
        if col >1: MaxLineLeft = max(lines[row][:col])
        else : MaxLineLeft = lines[row][0]
        if col+1 <= sizeLine-1 : MaxLineRight = max(lines[row][col+1:])
        else: MaxLineRight = lines[row][sizeLine-1]

        if lines[row][col] > MaxColumnBottom:
            bottomDistance = nbLines-(row+1)
        else :
            something = [index + row+1 for (index, item) in enumerate(columns[col][row+1:]) if item >= lines[row][col]]
            bottomDistance = something[0] - (row)

        if lines[row][col] > MaxColumnTop:
            topDistance = row
        else :
            something = [index for (index, item) in reversed(list(enumerate(columns[col][:row]))) if item >= lines[row][col]]
            topDistance = (row)-something[0]

        if lines[row][col] > MaxLineLeft:
            leftDistance = col
        else :
            something = [index for (index, item) in reversed(list(enumerate(lines[row][:col]))) if item >= lines[row][col]]
            leftDistance = (col)-something[0]

        if lines[row][col] > MaxLineRight:
            rightDistance = sizeLine - (col+1)
        else :
            something = [index+col+1 for (index, item) in enumerate(lines[row][col+1:]) if item >= lines[row][col]]
            rightDistance = something[0] - (col)

        totalScoreTree = topDistance*bottomDistance*leftDistance*rightDistance

        scores.append(totalScoreTree)
            
result = max(scores)
print(f"Max scenic view : {result}")