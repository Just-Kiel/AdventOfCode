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
answer = 0

for i, block in enumerate(part1):
    a, b = map(eval, block.split("\n"))
    if comparison(a, b) == 1:
        answer+=i+1

print(f"Sum of lines : {answer}")
# expected : 5350