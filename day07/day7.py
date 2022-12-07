from dataclasses import dataclass
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
                    # print(Lines[j][5:-1])
                    print(f"{Lines[i]} {Lines[j]}")
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

path = ''
for i in range(0, len(Lines), 1):
    # is a command
    currentLine = Lines[i][:-1]
    if "$ cd" in currentLine and currentLine != "$ cd ..":
        folderLists.append(sumInFolder(i))

folderLists = sumOfFolders(folderLists)

final = sumOfDeletion(100000, folderLists)

print(f"Sum to delete : {final}")
# expected : 1447046