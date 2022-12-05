import queue

print ("Day Five - Part One !")

queues = [queue.LifoQueue() for i in range(9)]

fichier = open("day05/input.txt", "r")
Lines = fichier.readlines()

def filterQueue(queue):
    for i in range(0, queue.qsize()):
        tempItem = queue.get()
        if tempItem != " ":
            queue.put(tempItem)
            break

def getInfoOfInstructions(line):
    line = line.split("move ")
    line = line[1].split(" from")
    tempLine = line[1].split(" to")
    line = line[0]+tempLine[0]+tempLine[1][:-1]

    line = line.split(' ')
    infos = [int(info) for info in line]
    return infos

# IN RIGHT ORDER
[[queues[x].put(Lines[i][1+4*x]) for i in range(7, -1, -1)] for x in range(0, len(queues))]

[filterQueue(queue) for queue in queues]

for line in Lines[10:]:
    infos = getInfoOfInstructions(line)
    for i in range(infos[0]):
        queues[infos[2]-1].put(queues[infos[1]-1].get())

result = [queue.get() for queue in queues]

# expected result : TBVFVDZPN
print('Top crates : '+''.join(result))


print ("Day Five - Part Two !")
queues2 = [queue.LifoQueue() for i in range(9)]
# IN RIGHT ORDER
[[queues2[x].put(Lines[i][1+4*x]) for i in range(7, -1, -1)] for x in range(0, len(queues2))]
[filterQueue(queue) for queue in queues2]

j=0
for line in Lines[10:]:
    infos = getInfoOfInstructions(line)
    tempQueue = queue.LifoQueue()
    for i in range(infos[0]):
        tempQueue.put(queues2[infos[1]-1].get())

    for i in range(infos[0]):
        queues2[infos[2]-1].put(tempQueue.get())

result2 = [queue.get() for queue in queues2]

# expected result : VLCWHTDSZ
print('Top crates in 2nd : '+''.join(result2))