use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn map_input(input: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    
    for c in input.chars() {
        if map.get(&c) != None {
            map.entry(c).and_modify(|x| *x += 1);
        } else {
            map.insert(c, 1);
        }
    }
    map
}

fn count_vowels(map: HashMap<char, u32>) -> bool {
    let count = 0;
    
    false
}

fn part1(input: &str) -> bool {
    let map = map_input(input);
    dbg!(map);
    
    true
}

fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "ugknbfddgicrmopn";
        let result = part1(input);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_part2() {
        let input = "abcdef";
        let result = part2(input);
        assert_eq!(result, 609043);
    }
}