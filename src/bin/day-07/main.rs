use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn parse_input(input: &str) -> HashMap<i64, Vec<Vec<i64>>> {
    let mut result: HashMap<i64, Vec<Vec<i64>>> = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let values: Vec<i64> = split[1].split_whitespace().collect::<Vec<&str>>().iter().map(|n| n.parse().unwrap()).collect();
        result.entry(split[0].parse().unwrap()).and_modify(|val| val.push(values.clone())).or_insert(vec![values]);
    }

    result
}

fn part1(input: &str) -> i64 {
    let results = parse_input(input);
    let mut total: i64 = 0;

    for (expected, posibilities) in results {
		for values in posibilities {
			let combinations = 2_i32.pow((values.len()-1) as u32);

			for i in 0..combinations {
				let mut current_calculation = 0;

				for (index, v) in values.iter().enumerate() {
					if index == 0 {
						current_calculation += v;
					} else {
						if (i >> index-1) & 1 == 0 {
							current_calculation += v;
						} else {
							current_calculation *= v;
						}
					}
				}

				if expected == current_calculation {
					total += expected;
					break;
				}
			}
		}
    }

    total
}

fn part2(input: &str) -> i64 {
    let results = parse_input(input);
	let mut total: i64 = 0;

	todo!();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = part1(input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn it_works_part2() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = part2(input);
        assert_eq!(result, 11387);
    }
}