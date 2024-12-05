use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut possibilities: Vec<Vec<i32>> = vec![];

    for line in input.lines() {
        if line == "" {
            continue;
        }

        if line.contains("|") {
            let split: Vec<&str> = line.split("|").collect();
            result.entry(split[1].parse().unwrap()).and_modify(|value| value.push(split[0].parse().unwrap())).or_insert(vec![split[0].parse().unwrap()]);
        } else {
            let split: Vec<i32> = line.split(",").map(|value| value.parse::<i32>().unwrap()).collect();
            possibilities.push(split);
        }
    }

    (result, possibilities)
}

fn part1(input: &str) -> i32 {
    let (results, possibilities) = parse_input(input);
    let mut total = 0;
    let empty = Vec::new();

    for possibility in possibilities {
        let mut iterate = true;
        for (i, nb) in possibility.iter().enumerate() {
            let temp = results.get(nb).unwrap_or(&empty);
            for count in 0..possibility.len()-i {
                if temp.contains(&possibility[i+count]) {
                    iterate = false;
                    break;
                }
            }
        }
        if iterate {
            total += possibility[possibility.len()/2];
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let (results, possibilities) = parse_input(input);
    let mut total = 0;
    let empty = Vec::new();

    for possibility in possibilities {
        let mut iterate = true;
        for (i, nb) in possibility.iter().enumerate() {
            let temp = results.get(nb).unwrap_or(&empty);
            for count in 0..possibility.len()-i {
                if temp.contains(&possibility[i+count]) {
                    iterate = false;
                    break;
                }
            }
        }
        if !iterate {
            let mut reordered: Vec<i32> = possibility.clone();

            let mut i = 0;
            while i < reordered.len() {
                let nb = reordered[i];
                let temp = results.get(&nb).unwrap_or(&empty);
                for index in i..reordered.len() {
                    if temp.contains(&reordered[index]) {
                        reordered.swap(i, index);
                        break;
                    }

                    if index == reordered.len()-1 {
                        i += 1;
                    }
                }
            }

            total += reordered[reordered.len()/2];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = part1(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn it_works_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

97,13,75,29,47";
        let result = part2(input);
        assert_eq!(result, 47);
    }
}