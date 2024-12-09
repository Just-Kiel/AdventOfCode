use std::{u32, usize::MAX};

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    // let part2 = part2(input);
    // dbg!(part2);
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut result = Vec::new();

    for value in input.chars() {
        result.push(value.to_digit(10).unwrap());
    }

    result
}

fn part1(input: &str) -> i64 {
    let results = parse_input(input);
    let mut total = 0;
    let mut vector: Vec<i32> = Vec::new();

    for (index, value) in results.iter().enumerate() {
        if index % 2 == 0 {
            for _i in 0..*value {
                vector.push(index as i32 /2);
            }
        } else {
            for _i in 0..*value {
                vector.push(-1);
            }
        }
    }

    let mut current_index = 0;
    while current_index < vector.len()-1 {
        if vector[current_index] != -1 {
            current_index += 1;
            continue;
        }

        let size = vector.len()-1;

        vector.swap(current_index, size);
        vector.remove(size);
    }

    for (index, val) in vector.iter().enumerate() {
        if *val == -1 {
            continue;
        }
        total += index as i64 * *val as i64;
    }

    total
}

fn part2(input: &str) -> i64 {
    let mut results = parse_input(input);
    let mut total = 0;
    let mut vector: Vec<i32> = Vec::new();

    todo!();

    for (index, value) in results.iter().enumerate() {
        if index % 2 == 0 {
            for _i in 0..*value {
                vector.push(index as i32 /2);
            }
        } else {
            for _i in 0..*value {
                vector.push(-1);
            }
        }
    }

    // Thanks to results, i can swap packs
    let mut size = vector.len();
    let max_id = (results.len()-1)/2;
    let mut id_computer: i32 = max_id as i32;
    while id_computer >= 0 {
        dbg!(id_computer);
        dbg!(vector.iter().position(|n| *n==id_computer));
        dbg!(vector.iter().filter(|n| **n==id_computer).count());
        let clone = results.clone();
        let pack_size = clone.last().unwrap();

        if *pack_size == 0 {
            results.remove(results.len()-1);
            results.remove(results.len()-1);
            id_computer -= 1;
            continue;
        }

        // find position of first element with this occurence and is odd number
        let elements = results.iter().enumerate().position(|(index, val)| val >= pack_size && index % 2 == 1).unwrap_or(MAX);
        if elements == MAX {
            if id_computer ==2 {
                dbg!(results.clone());
            }
            results.remove(results.len()-1);
            size -= (*pack_size as usize) + *results.last().unwrap() as usize;
            results.remove(results.len()-1);
            id_computer -= 1;
            continue;
        }
        let index = results[..elements].iter().sum::<u32>();

        for el in 0..*pack_size {
            vector.swap((el+ index) as usize, size-(*pack_size-el) as usize);
        }
        results[elements-1] += pack_size;
        results[elements] -= pack_size;
        results.remove(results.len()-1);
        size -= (*pack_size as usize) + *results.last().unwrap() as usize;
        results.remove(results.len()-1);

        dbg!(vector.clone());

        id_computer -= 1;
    }

    for (index, val) in vector.iter().enumerate() {
        if *val == -1 {
            continue;
        }
        total += index as i64 * *val as i64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2333133121414131402";
        let result = part1(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn it_works_part2() {
        let input = "2333133121414131402";
        let result = part2(input);
        assert_eq!(result, 2858);
    }
}