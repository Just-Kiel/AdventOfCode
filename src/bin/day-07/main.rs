use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = parse_input(input);
    let result1 = part1(ordered_lists.clone());
    dbg!(result1);
    let ordered_lists2 = parse_input(input);
    let part2 = part2(ordered_lists2);
    dbg!(part2);
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, usize) {
    // get S in line 0
    // get all ^ in all lines
    // so best use hash map
    
    let mut map_letters = HashMap::new();

    for (i_line, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            map_letters.entry(c).and_modify(|value: &mut Vec<(i32, i32)>| value.push((index.try_into().unwrap(), i_line.try_into().unwrap()))).or_insert(vec![(index.try_into().unwrap(), i_line.try_into().unwrap())]);
        }
    }

    (map_letters, input.lines().count())
}

fn part1(input: (HashMap<char, Vec<(i32, i32)>>, usize)) -> i64 {
    let mut total = 0;

    // get S position
    let s_position = input.0.get(&'S').unwrap()[0];
    let split_positions = input.0.get(&'^').unwrap();

    let mut current_positions = Vec::new();
    current_positions.push(s_position);

    for line in 0..input.1 {
        let mut new_positions = Vec::new();

        for current in current_positions.clone() {
            if split_positions.contains(&(current.0, current.1 + 1)) {
                if !new_positions.contains(&(current.0 - 1, current.1 + 1)){
                    new_positions.push((current.0 - 1, current.1 + 1));
                }
                if !new_positions.contains(&(current.0 + 1, current.1 + 1)){
                    new_positions.push((current.0 + 1, current.1 + 1));
                }
                total+=1;
            } else {
                if !new_positions.contains(&(current.0, current.1 + 1)){
                    new_positions.push((current.0, current.1 + 1));
                }
            }
        }
        current_positions = new_positions;
    }

    total
}

fn part2(input: (HashMap<char, Vec<(i32, i32)>>, usize)) -> i64 {
    let mut total = 0;

    // get S position
    let s_position = input.0.get(&'S').unwrap()[0];
    let split_positions = input.0.get(&'^').unwrap();

    let mut current_positions = Vec::new();
    current_positions.push(s_position);

    for line in 0..input.1 {
        let mut new_positions = Vec::new();

        for current in current_positions.clone() {
            if split_positions.contains(&(current.0, current.1 + 1)) {
                if !new_positions.contains(&(current.0 - 1, current.1 + 1)){
                    new_positions.push((current.0 - 1, current.1 + 1));
                }
                if !new_positions.contains(&(current.0 + 1, current.1 + 1)){
                    new_positions.push((current.0 + 1, current.1 + 1));
                }
                total+=2;
            } else {
                if !new_positions.contains(&(current.0, current.1 + 1)){
                    new_positions.push((current.0, current.1 + 1));
                }
            }
        }
        current_positions = new_positions;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let order = parse_input(input);
        let result = part1(order);
        assert_eq!(result, 21);
    }

    #[test]
    fn it_works_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let order = parse_input(input);
        let result = part2(order);
        assert_eq!(result, 40);
    }
}