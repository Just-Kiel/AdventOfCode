# use of dictionnary for H and T to know the movement
# stack as static value the s ?
# count all different occurences in T

# if H left : T[-1][]col-1=H[-1][]col : T[-1][]col ++

from dataclasses import dataclass
print ("Day Nine - Part One !")

fichier = open("day09/input.txt", "r")
fichier = fichier.read()

lines = [x for x in fichier.split('\n')]

#to enable on real input
lines = lines[:-1]

@dataclass
class Point:
  row: int
  col: int
  def __add__(self, o):
        return Point(self.row + o.row, self.col + o.col)
    

head = Point(0, 0)
tail = Point(0, 0)

right = Point(0, 1)
left = Point(0, -1)
up = Point(1, 0)
down = Point(-1, 0)

tailMoves = []

for line in lines:
    moves = [x for x in line.split(' ')]

    moves = [moves[0] for i in range(int(moves[1]))]

    # set moves of head
    for move in moves:
        if move == 'R':
            head += right
        elif move == 'L':
            head += left
        elif move == 'U':
            head += up
        elif move == 'D':
            head += down

        if abs(head.col - tail.col) + abs(head.row-tail.row) > 2:
            # si h.row > tail.row and h.col > tail.col : U + R
            # si h.row > tail.row and tail.col > h.col : U + L
            # si tail.row > h.row and h.col > tail.col : D + R
            # else : D + L

            if head.row > tail.row and head.col > tail.col:
                tail += up + right
            elif head.row > tail.row and head.col < tail.col:
                tail += up + left
            elif tail.row > head.row and head.col > tail.col:
                tail += down + right
            else:
                tail += down + left
        elif abs(head.col - tail.col) + abs(head.row-tail.row) > 1:
            if head.row == tail.row:
                if head.col > tail.col:
                    tail+=right
                else:
                    tail+=left
                
            elif head.col == tail.col:
                if head.row > tail.row:
                    tail+=up
                else:
                    tail+=down
        tailMoves.append(tail)

print(len(tailMoves))

new_list = []

[new_list.append(item) for item in tailMoves if item not in new_list]
print(f"Positions visited : {len(new_list)}")
# expected : 5878

# ---------------------------------
print ("Day Nine - Part Two !")

rope = [Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0), Point(0, 0)]

tailMoves2 = []

for line in lines:
    moves = [x for x in line.split(' ')]

    moves = [moves[0] for i in range(int(moves[1]))]

    # set moves of head
    for move in moves:
        if move == 'R':
            rope[0] += right
        elif move == 'L':
            rope[0] += left
        elif move == 'U':
            rope[0] += up
        elif move == 'D':
            rope[0] += down

        for i in range(1, len(rope)):
            if abs(rope[i-1].col - rope[i].col) + abs(rope[i-1].row-rope[i].row) > 2:
                # si h.row > tail.row and h.col > tail.col : U + R
                # si h.row > tail.row and tail.col > h.col : U + L
                # si tail.row > h.row and h.col > tail.col : D + R
                # else : D + L

                if rope[i-1].row > rope[i].row and rope[i-1].col > rope[i].col:
                    rope[i] += up + right
                elif rope[i-1].row > rope[i].row and rope[i-1].col < rope[i].col:
                    rope[i] += up + left
                elif rope[i].row > rope[i-1].row and rope[i-1].col > rope[i].col:
                    rope[i] += down + right
                else:
                    rope[i] += down + left
            elif abs(rope[i-1].col - rope[i].col) + abs(rope[i-1].row-rope[i].row) > 1:
                if rope[i-1].row == rope[i].row:
                    if rope[i-1].col > rope[i].col:
                        rope[i]+=right
                    else:
                        rope[i]+=left
                    
                elif rope[i-1].col == rope[i].col:
                    if rope[i-1].row > rope[i].row:
                        rope[i]+=up
                    else:
                        rope[i]+=down
        tailMoves2.append(rope[9])

print(len(tailMoves2))

new_list = []

[new_list.append(item) for item in tailMoves2 if item not in new_list]
print(f"Positions visited with rope[9] : {len(new_list)}")
# expected : 2405