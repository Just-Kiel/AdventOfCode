fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input, part1);
    dbg!(part2);
}

fn part1(input: &str) -> u32 {
    let mut iteration = 0;
    let detection= "00000";
    while format!("{:x}", md5::compute(input.to_owned() + &iteration.to_string())).split_at(5).0 != detection {
        iteration += 1;
    }

    iteration
}

fn part2(input: &str, start: u32) -> u32 {
    let mut iteration = start;
    let detection= "000000";
    while format!("{:x}", md5::compute(input.to_owned() + &iteration.to_string())).split_at(6).0 != detection {
        iteration += 1;
    }

    iteration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "abcdef";
        let result = part1(input);
        assert_eq!(result, 609043);
    }

    #[test]
    fn it_works_part2() {
        let input = "abcdef";
        let result = part2(input, 0);
        assert_eq!(result, 609043);
    }
}