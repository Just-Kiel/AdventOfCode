from dataclasses import dataclass
from collections import defaultdict
print ("Day Seven - Part One !")

fichier = open("day07/input.txt", "r")
Lines = fichier.readlines()

@dataclass
class Folder:
  name: str
  sum: int
  contains: list[str]

def sumInFolder(index) -> Folder:
    name = index
    sum = 0
    content = []
    for i in range(index+2, len(Lines), 1):
        if "$ cd" in Lines[i]: break
        if not 'dir' in Lines[i]: 
            endGetter = Lines[i].find(' ')
            sum+=int(Lines[i][0:endGetter])
        else:
            # not i but next occurence
            for j in range(i+1, len(Lines), 1):
                if "$ cd "+Lines[i][4:-1] in Lines[j][:-1]:
                    content.append(j)
                    break
    return Folder(name, sum, content)
        
def sumOfFolders(folders):
    # print(folders)
    for i in range(len(folders)-1, -1, -1):
        if folders[i].contains:
            toAdd = [subfolder.sum for subfolder in folders if subfolder.name in folders[i].contains]

            folders[i].sum += sum(toAdd)
    return (folders)

def sumOfDeletion(maxSize, folders):
    sumToDelete = 0
    for folder in folders:
        if folder.sum <= maxSize:
            sumToDelete += folder.sum
    return sumToDelete

folderLists = []

for i in range(0, len(Lines), 1):
    # is a command
    currentLine = Lines[i][:-1]
    if "$ cd" in currentLine and currentLine != "$ cd ..":
        folderLists.append(sumInFolder(i))

folderLists = sumOfFolders(folderLists)

final = sumOfDeletion(100000, folderLists)

print(f"Sum to delete : {final}")
# expected : 1447046

# --------------------------------
print ("Day Seven - Part Two !")

print(f"Total root with my function (seems like there is an error) : {folderLists[0].sum}")
# expected : 40 572 957


# look there to understand : https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/7.py
path = []
SZ = defaultdict(int)
for line in Lines:
    part = line[:-1].split(' ')
    # print(words)

    if part[1] == "cd":
        if part[2] == "..":
            # remove last item
            path.pop()
        else:
            path.append(part[2])
    elif part[1] == 'ls':
        continue
    elif part[0] == 'dir':
        continue
    else:
        sum = int(part[0])

        for i in range(1, len(path)+1):
            SZ['/'.join(path[:i])] += sum

print(f"Total root : {SZ['/']}")

maxUsedForUpdate = 70000000 - 30000000
needeedSpace =SZ['/'] - maxUsedForUpdate

SZ = list(sorted(SZ.items(), key=lambda item: item[1]))
SZ = list(filter(lambda item: item[1] >= needeedSpace, SZ))

print(f"Size of folder to delete : {SZ[0][1]}")