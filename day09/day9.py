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
