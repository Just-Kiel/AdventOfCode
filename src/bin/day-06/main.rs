use std::{collections::HashMap, ops::AddAssign, sync::{Arc, Mutex}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1.0);
    let part2 = part2(input, part1.1);
    dbg!(part2);
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, (usize, usize)) {
    let mut result: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (i_line, line) in input.lines().enumerate() {
        for (index, c) in line.chars().enumerate() {
            result.entry(c).and_modify(|value: &mut Vec<(i32, i32)>| value.push((index.try_into().unwrap(), i_line.try_into().unwrap()))).or_insert(vec![(index.try_into().unwrap(), i_line.try_into().unwrap())]);
        }
    }

    (result, (width, height))
}

fn part1(input: &str) -> (i32, Vec<(i32, i32)>) {
    let (results, size) = parse_input(input);

    let mut current_point = results.get(&'^').unwrap()[0];

    let prohibited_positions = results.get(&'#').unwrap();
    let mut visited_positions: Vec<(i32, i32)> = vec![current_point];

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_direction = 0;

    let mut keep_moving = true;

    while keep_moving {
        let temp_point = (current_point.0 + directions[current_direction].0, current_point.1 + directions[current_direction].1);
        if prohibited_positions.contains(&temp_point) {
            if current_direction < 3 {
                current_direction += 1;
            } else {
                current_direction = 0;
            }
        } else {
            current_point = temp_point;
            
            // stop condition
            if current_point.1 > (size.1 as i32)-1 || current_point.0 > (size.0 as i32)-1 || current_point.0 < 0 || current_point.1 < 0 {
                keep_moving = false;
                continue;
            }

            if !visited_positions.contains(&current_point) {
                visited_positions.push(current_point);
            }
        }
    }

    (visited_positions.len() as i32, visited_positions)
}

fn part2(input: &str, visited: Vec<(i32, i32)>) -> i32 {
    let (results, size) = parse_input(input);
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let prohibited_positions = results.get(&'#').unwrap().clone();
    // create total as mutex
    let total = Arc::new(Mutex::new(0));

    visited.par_iter().for_each(|&pos| {
        let mut current_direction = 0;
        let mut current_point = results.get(&'^').unwrap()[0];
        if current_point == pos {
            return;
        }
        let mut keep_moving = true;
        let mut visited_positions: Vec<((i32, i32), usize)> = vec![(current_point, current_direction)];
        let mut modified_prohibited_positions = prohibited_positions.clone();
        modified_prohibited_positions.push(pos);
    

        while keep_moving {
            let temp_point = (current_point.0 + directions[current_direction].0, current_point.1 + directions[current_direction].1);
            if modified_prohibited_positions.contains(&temp_point) {
                if current_direction < 3 {
                    current_direction += 1;
                } else {
                    current_direction = 0;
                }
            } else {
                current_point = temp_point;
                
                // stop condition
                if current_point.1 > (size.1 as i32)-1 || current_point.0 > (size.0 as i32)-1 || current_point.0 < 0 || current_point.1 < 0 {
                    keep_moving = false;
                    continue;
                }

                if !visited_positions.contains(&(current_point, current_direction)) {
                    visited_positions.push((current_point, current_direction));
                } else {
                    total.lock().unwrap().add_assign(1);
                    keep_moving = false;
                    continue;
                }
            }
        }
    });

    let total_live_longer = total.lock().unwrap().clone();
    total_live_longer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = part1(input);
        assert_eq!(result.0, 41);
    }

    #[test]
    fn it_works_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let part1 = part1(input);
        let result = part2(input, part1.1);
        assert_eq!(result, 6);
    }
}