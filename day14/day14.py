print ("Day Fourteen - Part One !")
fichier = open("day14/test.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]
lines = lines[:-1]

map = {}
# add enter of sand
map[(500, 0)] = '+'

map2 = {}

def fillMap(lines, map):
    for line in lines:
        line = line.split(' -> ')

        x, y = eval(line[0])
        map[(x,y)] = '#' # contains rock

        for i in range(1, len(line)):
            x, y = eval(line[i])

            xPrevious, yPrevious = eval(line[i-1])

            xToIter = []
            xToIter.append(x)

            for j in range(0, x-xPrevious, -1) if x-xPrevious <0 else range(x-xPrevious):
                xToIter.append(x-j)

            yToIter = []
            yToIter.append(y)
            for j in range(0, y-yPrevious, -1) if y-yPrevious <0 else range(y-yPrevious):
                yToIter.append(y-j)

            for a in range(len(xToIter)):
                for b in range(len(yToIter)):
                    map[(xToIter[a],yToIter[b])] = '#' # contains rock

    return map

map = fillMap(lines, map)

sandEnter = list(map.keys())[list(map.values()).index('+')]

yMax = max([y for (x, y) in map.keys()])

def sandFall(mapTemp, yMaxTemp):
    x, y = sandEnter

    while y <= yMaxTemp:
        if (not (x, y+1) in mapTemp or ((mapTemp[(x, y+1)] != '#') and (mapTemp[(x, y+1)] != 'o'))):
            y+=1
            continue

        if not (x-1, y+1) in mapTemp or ((mapTemp[(x-1, y+1)] != '#') and (mapTemp[(x-1, y+1)] != 'o')):
            x-=1
            y+=1
            continue

        if not (x+1, y+1) in mapTemp or ((mapTemp[(x+1, y+1)] != '#') and (mapTemp[(x+1, y+1)] != 'o')):
            x+=1
            y+=1
            continue
        
        print("oui")
        mapTemp[(x, y)] = 'o'
        print(f"({x}, {y})")
        return True
    return False


while True:
    result = sandFall(map, yMax)

    if not result:
        break

result = [x for x in map.values() if x == "o"]
print(f"Résultat partie 1 : {len(result)}")
# expected : 843


print ("Day Fourteen - Part Two !")
map2 = fillMap(lines, map2)

def sandFall2(mapTemp, yMaxTemp):
    x, y = (500, 0)

    if (x, y) in mapTemp:
        return (x, y)

    while y <= yMaxTemp:
        if (not (x, y+1) in mapTemp or ((mapTemp[(x, y+1)] != '#') and (mapTemp[(x, y+1)] != 'o'))):
            y+=1
            continue

        if not (x-1, y+1) in mapTemp or ((mapTemp[(x-1, y+1)] != '#') and (mapTemp[(x-1, y+1)] != 'o')):
            x-=1
            y+=1
            continue

        if not (x+1, y+1) in mapTemp or ((mapTemp[(x+1, y+1)] != '#') and (mapTemp[(x+1, y+1)] != 'o')):
            x+=1
            y+=1
            continue
        
        break
    return (x, y)

xMax = max([x for (x, y) in map2.keys()])
xMin = min([x for (x, y) in map2.keys()])

xToIter = []
xToIter.append(xMin)

for j in range(xMax-xMin):
    xToIter.append(xMax-j)
print(xToIter)

for a in range(len(xToIter)):
    map2[(xToIter[a], yMax+2)] = '#' # contains rock

yMax2= max([y for (x, y) in map2.keys()])-1
print(yMax2)

print(map2)

def simulate_sand():
    x, y = 500, 0

    if (x, y) in map2:
        return (x, y)

    while y <= yMax2:
        if (x, y + 1) not in map2:
            y += 1
            continue

        if (x - 1, y + 1) not in map2:
            x -= 1
            y += 1
            continue

        if (x + 1, y + 1) not in map2:
            x += 1
            y += 1
            continue

        # Everything filled, come to rest
        break

    return (x, y)

while True:
    a, b = simulate_sand()

    map2[(a, b)] = 'o'

    if (a, b) == (500, 0):
        break

print(map2)

result2 = [x for x in map2.values() if x == "o"]
print(f"Résultat partie 2 : {len(result2)}")