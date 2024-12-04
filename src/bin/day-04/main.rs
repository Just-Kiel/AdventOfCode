use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
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

fn test_right(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0+(letter_count as i32)+1, start.1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_left(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0-(letter_count as i32)-1, start.1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_down(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0, start.1+(letter_count as i32)+1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_up(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0, start.1-(letter_count as i32)-1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_right_down(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0+(letter_count as i32)+1, start.1+(letter_count as i32)+1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_right_up(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0+(letter_count as i32)+1, start.1-(letter_count as i32)-1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_left_down(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0-(letter_count as i32)-1, start.1+(letter_count as i32)+1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn test_left_up(start: &(i32, i32), positions: &Vec<&Vec<(i32, i32)>>, max_count: usize) -> bool {
    let mut letter_count = 0;

    while letter_count < max_count {
        if positions[letter_count].iter().filter(|value| **value == (start.0-(letter_count as i32)-1, start.1-(letter_count as i32)-1)).count() == 1 {
            letter_count += 1;
        } else {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> i32 {
    let results = parse_input(input);
    let mut total = 0;

    // access to X and look directionally
    let start_positions = results.get(&'X').unwrap();
    let positions = vec![results.get(&'M').unwrap(), results.get(&'A').unwrap(), results.get(&'S').unwrap()];

    // XMAS
    for pos in start_positions {
        // Look right (x+1, y)
        // Look Left
        // Look down
        // Look Up
        if test_right(pos, &positions, 3) {
            total += 1;
        }
        if test_left(pos, &positions, 3) {
            total += 1;
        }

        if test_down(pos, &positions, 3) {
            total += 1;
        }
        if test_up(pos, &positions, 3) {
            total += 1;
        }


        // Look Right Down
        // Look Right Up
        if test_right_down(pos, &positions, 3) {
            total += 1;
        }
        if test_right_up(pos, &positions, 3) {
            total += 1;
        }
        // Look Left down
        // Look Left Up
        if test_left_down(pos, &positions, 3) {
            total += 1;
        }
        if test_left_up(pos, &positions, 3) {
            total += 1;
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    let results = parse_input(input);

    // ACCESS to M
    // Look if some are separated by 1 by side or by up and down
    let start_positions = results.get(&'M').unwrap();
    let positions = vec![results.get(&'A').unwrap(), results.get(&'S').unwrap()];
    let max_count = 2;

    for pos in start_positions {
        if test_right_down(pos, &positions, max_count) {
            if start_positions.iter().filter(|value| **value == (pos.0+2, pos.1)).count() == 1 && test_left_down(&(pos.0+2, pos.1), &positions, max_count) {
                total += 1;
            }

            if start_positions.iter().filter(|value| **value == (pos.0, pos.1+2)).count() == 1 && test_right_up(&(pos.0, pos.1+2), &positions, max_count) {
                total += 1;
            }
        }

        if test_left_down(pos, &positions, max_count) {
            // left_up
            if start_positions.iter().filter(|value| **value == (pos.0, pos.1+2)).count() == 1 && test_left_up(&(pos.0, pos.1+2), &positions, max_count) {
                total += 1;
            }
        }

        if test_right_up(pos, &positions, max_count) {
            if start_positions.iter().filter(|value| **value == (pos.0+2, pos.1)).count() == 1 && test_left_up(&(pos.0+2, pos.1), &positions, max_count) {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = part1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn it_works_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = part2(input);
        assert_eq!(result, 9);
    }
}