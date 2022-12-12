from more_itertools import locate
print ("Day Twelve - Part One !")
fichier = open("day12/input.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]
part2 = [x for x in fichier.split('\n')]
fichier = fichier.replace('\n', '')

# find S : done
# find E : done
# find next up around current : https://python.plainenglish.io/difficult-python-question-9-shortest-paths-in-a-maze-361d51c53575

whereS = fichier.find("S")
whereE = fichier.find("E")

sizeLine = len(lines[0])

def convertMap(map):
    map = [list(line) for line in map]
    map[whereS//sizeLine][whereS%sizeLine] = 'a'
    map[whereE//sizeLine][whereE%sizeLine] = 'z'

    for i in range(len(map)):
        for j in range(sizeLine):
            map[i][j] = ord(map[i][j])-ord('a')

    return map

def findShortPath(maze, xStart, yStart, end):
    d = {
        (yStart,xStart):{"shortest distance": 0, "previous":None}
    }

    queue = [(yStart,xStart)]
    # target = []
    target = (0,0)

    # while queue not empty
    while len(queue) > 0:
        # first element in queue
        i, j = queue.pop(0)
        previous = (i, j)
        shortest_distance = d[(i,j)]["shortest distance"]

        # searching in 4 directions - up, down, left, right
        for di,dj in [(0,-1),(-1,0),(0,1),(1,0)]:
            # (ni,nj) is the new coordinate here
            ni,nj = i+di, j+dj

            # ignore (ni,nj) if it's out of bounds
            if ni<0 or nj<0:
                continue

            # ignore (ni,nj) if it's out of bounds
            if ni>=len(maze) or nj>=len(maze[0]):
                continue

            # ignore (ni,nj) if it's not a valid coordinate
            if maze[ni][nj] > maze[i][j]+1:
                continue

            # if (ni,nj) is not inside d, add it to d
            # then add (ni,nj) back to queue so we can search it later
            if (ni,nj) not in d:
                d[(ni,nj)] = {"shortest distance": shortest_distance+1, "previous": previous}
                queue.append((ni,nj))

            # if (ni,nj) is already inside d, check if the new "shortest distance" is shorter than the orignal
            # update the original only if the new shorter distance is in fact shorter
            else:
                original_shortest_distance = d[(ni,nj)]["shortest distance"]
                if shortest_distance + 1 < original_shortest_distance:
                    d[(ni,nj)] = {"shortest distance": shortest_distance+1, "previous": previous}
            
            # if coordinate points to "E", we add it to targets
            if ni == whereE//sizeLine and nj == whereE%sizeLine:
                target = end
            
    # at this point, queue is empty, and we have searched all possible points inside the maze
    out = []
    if target == end:
        # print(target)
        # constructing shortest paths based on d
        path = []
        current = target
        while current != None:
            path.append(current)
            current = d[current]["previous"]
        out = path[::-1]

    return out


charS = lines[whereS//sizeLine][whereS%sizeLine]
charE = lines[whereE//sizeLine][whereE%sizeLine]
end = (whereE//sizeLine, whereE%sizeLine)

# map convertie
lines = convertMap(lines)

lines = findShortPath(lines, whereS%sizeLine, whereS//sizeLine, end)

result = len(lines)-1
print(f"Number of steps : {result}")
# expected : 380

print ("Day Twelve - Part Two !")

possibleStart = list(locate(fichier, lambda x: x == 'a'))
possibleStart.append(whereS)

possibleStart = [(value%sizeLine, value//sizeLine) for value in possibleStart]

part2 = convertMap(part2)

result = [len(findShortPath(part2, x, y, end))-1 for (x,y) in possibleStart]

result = filter(lambda x: x != -1, result)

result = sorted(result)

print(f"Shortest from other start : {result[0]}")
# expected : 375