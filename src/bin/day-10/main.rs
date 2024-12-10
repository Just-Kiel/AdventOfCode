use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<(i32, i32)>>, (usize, usize)) {
    let mut result: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (i_line, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            result.entry(c.to_digit(10).unwrap() as i32).and_modify(|value: &mut Vec<(i32, i32)>| value.push((index.try_into().unwrap(), i_line.try_into().unwrap()))).or_insert(vec![(index.try_into().unwrap(), i_line.try_into().unwrap())]);
        }
    }

    (result, (width, height))
}

fn recursion_search(current_test_element: i32, start_positions: Vec<(i32, i32)>, all_positions: &HashMap<i32, Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut total: Vec<(i32, i32)> = Vec::new();

    if current_test_element == 10 {
        return start_positions;
    }

    for pos in start_positions {
        //up
        if all_positions.get(&current_test_element).unwrap().contains(&(pos.0, pos.1-1)) {
            // println!("{}:{:?}", current_test_element, (pos.0, pos.1-1));
            total.append(&mut recursion_search(current_test_element+1, vec![(pos.0, pos.1-1)], all_positions));
        }

        //down
        if all_positions.get(&current_test_element).unwrap().contains(&(pos.0, pos.1+1)) {
            // println!("{}:{:?}", current_test_element, (pos.0, pos.1+1));
            total.append(&mut recursion_search(current_test_element+1, vec![(pos.0, pos.1+1)], all_positions));
        }

        //left
        if all_positions.get(&current_test_element).unwrap().contains(&(pos.0-1, pos.1)) {
            // println!("{}:{:?}", current_test_element, (pos.0, pos.1+1));
            total.append(&mut recursion_search(current_test_element+1, vec![(pos.0-1, pos.1)], all_positions));
        }

        //right
        if all_positions.get(&current_test_element).unwrap().contains(&(pos.0+1, pos.1)) {
            // println!("{}:{:?}", current_test_element, (pos.0, pos.1+1));
            total.append(&mut recursion_search(current_test_element+1, vec![(pos.0+1, pos.1)], all_positions));
        }
    }

    total
}

fn part1(input: &str) -> i32 {
    let (results, size) = parse_input(input);
    let mut total = 0;
    
    let start_positions = results.get(&0).unwrap();
    let current_test_element = 1;
    
    for pos in start_positions {
        let mut vector: Vec<(i32, i32)> = Vec::new();
        vector.append(&mut recursion_search(current_test_element, vec![*pos], &results));
        total += vector.drain(..).collect::<HashSet<(i32, i32)>>().len();
    }

    total as i32
}

fn part2(input: &str) -> i32 {
    let (results, size) = parse_input(input);
    let mut total = 0;
    
    let start_positions = results.get(&0).unwrap();
    let current_test_element = 1;
    
    for pos in start_positions {
        let mut vector: Vec<(i32, i32)> = Vec::new();
        vector.append(&mut recursion_search(current_test_element, vec![*pos], &results));
        total += vector.len();
    }

    total as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = part1(input);
        assert_eq!(result, 36);
    }

    #[test]
    fn it_works_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = part2(input);
        assert_eq!(result, 81);
    }
}