from collections import defaultdict
import copy

import functools
import operator

print ("Day Eleven - Part One !")
fichier = open("day11/input.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]

nbMonkeys = 8

groups = [lines[1+(x*7):6+(x*7)] for x in range(nbMonkeys)]

monkeys = []
monkeys2 = []
for group in groups:
    group = [" ".join(s.split()) for s in group]
    oui = dict()
    oui["items"] = [int(group.replace(',',"")) for group in group[0].split(" ")[2:]]
    oui["operation"] = [group for group in group[1].split(" ")[4:]]
    if oui["operation"][1] != "old":
        oui["operation"][1] = int(oui["operation"][1])
    oui["test"] = int(group[2].split(" ")[3])
    oui["true"] = int(group[3].split(" ")[5])
    oui["false"] = int(group[4].split(" ")[5])
    oui["inspected"] = 0

    monkeys.append(oui)
    monkeys2.append(copy.deepcopy(oui))

# math magic trick <3 by Enguerrand
divider = functools.reduce(operator.mul, [monkey["test"] for monkey in monkeys])
# divider = functools.reduce(lambda acc, val: acc * val, [monkey["test"] for monkey in monkeys], 1)

# divider = 1
# for monkey in monkeys:
#     divider *= monkey["test"]

def do_operation(items, operation):
    if operation[0] == "*":
        if operation[1] == "old":
            items[0] *= items[0]
        else:
            items[0] *= operation[1]

    elif operation[0] == "+":
        if operation[1] == "old":
            items[0] += items[0]
        else:
            items[0] += operation[1]

def day11(rounds, value, monkeys):
    for i in range(rounds):
        for monkey in monkeys:
            for i in range(len(monkey["items"])):
                do_operation(monkey["items"], monkey["operation"])

                # gets bored
                monkey["items"][0] //= value

                # made by Enguerrand et Jules
                monkey["items"][0] %= divider
                bli = monkey["false"] if monkey["items"][0] % monkey["test"] != 0 else monkey["true"]
                monkeys[bli]["items"].append(monkey["items"].pop(0))
                monkey["inspected"]+=1

    return monkeys

part1 = day11(20, 3, monkeys)
inspected = [monkey["inspected"] for monkey in part1]

inspected = sorted(inspected, reverse=True)

result = inspected[0]*inspected[1]

print(f"Total : {result}")
# expected : 110888

print ("Day Eleven - Part Two !")
part2 = day11(10000, 1, monkeys2)
inspected = [monkey["inspected"] for monkey in part2]

inspected = sorted(inspected, reverse=True)

result = inspected[0]*inspected[1]

print(f"Total : {result}")
# expected : 25590400731