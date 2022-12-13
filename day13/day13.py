import functools
import operator
print ("Day Thirteen - Part One !")
fichier = open("day13/input.txt", "r")
fichier = fichier.read()

def comparison(a,b):
    if isinstance(a, list) and isinstance(b, int):
        b=[b]

    if isinstance(a, int) and isinstance(b, list):
        a=[a]

    if isinstance(a, int) and isinstance(b, int):
        if a < b :
            return 1
        if a == b:
            return 0
        return -1
    
    if isinstance(a, list) and isinstance(b, list):
        current = 0
        while current < len(a) and current < len(b):
            element = comparison(a[current], b[current])
            if element == 1:
                return 1
            if element == -1:
                return -1
            current+=1

        if current == len(a):
            if len(a) == len(b):
                return 0
            return 1
        return -1

part1 = fichier.strip().split("\n\n")
part2 = fichier.strip().replace("\n\n", "\n").split("\n")
part2 = list(map(eval, part2))
answer = 0

for i, block in enumerate(part1):
    a, b = map(eval, block.split("\n"))
    if comparison(a, b) == 1:
        answer+=i+1

print(f"Sum of lines : {answer}")
# expected : 5350

print ("Day Thirteen - Part Two !")
part2.append([[2]])
part2.append([[6]])

part2 = sorted(part2, key=functools.cmp_to_key(comparison), reverse=True)

mulMark2 =1
mulMark6 =1
for i, element in enumerate(part2):
    if element == [[2]]:
        mulMark2=i+1
    if element == [[6]]:
        mulMark6=i+1

print(f"Multiply of markers : {mulMark2*mulMark6}")
# expected : 19570