fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = order_lists(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn order_lists(input: &str) -> (Vec<(i64, i64)>) {
    let mut result: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let splitted= line.split(",").collect::<Vec<&str>>();
        for split in splitted {
            if split.is_empty() {
                continue;
            }
            let ranges = split.split("-").collect::<Vec<&str>>();
            result.push((ranges[0].parse().unwrap(), ranges[1].parse().unwrap()));
        }
    }
    result
}

fn part1(input: Vec<(i64, i64)>) -> i64 {
    let mut total = 0;

    for (range_start, range_end) in input {
        for number in range_start..=range_end {
            let number_str = number.to_string();
            let middle = number_str.chars().count()/2;
            let first_half = &number_str[0..middle];
            let second_half = &number_str[middle..];

            if first_half == second_half {
                total += number;
            }
        }
    }

    total
}

fn part2(input: Vec<(i64, i64)>) -> i128 {
    let mut total: i128 = 0;

    for (range_start, range_end) in input {
        for number in range_start..=range_end {
            let number_str = number.to_string();
            let count = number_str.chars().count();

            let mut iteration = 2;
            while iteration <= count {
                let vec_chars = number_str.chars().collect::<Vec<char>>();
                let size_of_part = count/iteration;
                let parts = vec_chars.windows(size_of_part).step_by(size_of_part);
                // Check if all parts are the same
                if parts.clone().collect::<Vec<&[char]>>().windows(2).all(|w| w[0] == w[1]) {
                    total += number as i128;
                    break;
                }
                iteration += 1;
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let order = order_lists(input);
        dbg!(&order);
        let result = part1(order);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn it_works_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let order = order_lists(input);
        let result = part2(order);
        assert_eq!(result, 4174379265);
    }
}