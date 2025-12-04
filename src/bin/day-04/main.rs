use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = parse_input(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn parse_input(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    // convert in a hash map with x and y
    let mut map_letters = HashMap::new();

    for (i_line, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            map_letters.entry(c).and_modify(|value: &mut Vec<(i32, i32)>| value.push((index.try_into().unwrap(), i_line.try_into().unwrap()))).or_insert(vec![(index.try_into().unwrap(), i_line.try_into().unwrap())]);
        }
    }

    map_letters
}

fn check_position(roll_positions:&Vec<(i32, i32)>, start_position: &(i32, i32), neighbor: (i32, i32)) -> bool {
    let neighbor_position = roll_positions.iter().find(|&&x| x == (start_position.0 + neighbor.0, start_position.1 + neighbor.1)).is_some();
    neighbor_position
}

fn part1(input: HashMap<char, Vec<(i32, i32)>>) -> i64 {
    let mut total = 0;

    let start_positions = input.get(&'@').unwrap();

    let checkable_positions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for position in start_positions {
        let mut local_count = 0;
        let mut iteration = 0;

        while local_count < 4 && iteration < 8 {
            if check_position(start_positions, position, checkable_positions[iteration]){
                local_count += 1;
            }

            iteration += 1;
        }

        if iteration == 8 && local_count < 4 {
            total += 1;
        }
    }

    total
}

fn part2(input: HashMap<char, Vec<(i32, i32)>>) -> i128 {
    let mut total = 0;

    let mut start_positions = input.get(&'@').unwrap().clone();

    let checkable_positions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let mut deletable_indexes = Vec::new();
    deletable_indexes.push(start_positions.iter().count());
    let mut deletable_size = !deletable_indexes.is_empty();

    while deletable_size {
        deletable_indexes.clear();
        deletable_size = false;
        for (index, position) in start_positions.iter().enumerate() {
            let mut local_count = 0;
            let mut iteration = 0;
            
            while local_count < 4 && iteration < 8 {
                if check_position(&start_positions, position, checkable_positions[iteration]){
                    local_count += 1;
                }
                
                iteration += 1;
            }
            
            if iteration == 8 && local_count < 4 {
                total += 1;
                
                deletable_indexes.push(index);
                deletable_size = true;
            }
        }

        deletable_indexes.reverse();
        for index in deletable_indexes.clone(){
            start_positions.remove(index);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let order = parse_input(input);
        let result = part1(order);
        assert_eq!(result, 13);
    }

    #[test]
    fn it_works_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let order = parse_input(input);
        let result = part2(order);
        assert_eq!(result, 43);
    }
}