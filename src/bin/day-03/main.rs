use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<(usize, i32, i32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut results: Vec<(usize, i32, i32)> = vec![];
    for (i, (_, [x, y])) in re.captures_iter(input).map(|c| (c.get(1).unwrap().start(), c.extract())) {
        results.push((i, x.parse().unwrap(), y.parse().unwrap()));
    }
    results
}

fn part1(input: &str) -> i32 {
    // mul(X,Y)
    let results = parse_input(input);

    let mut total = 0;

    for r in results {
        total += r.1*r.2;
    }

    total
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    let mut results = parse_input(input);

    let re = Regex::new(r"(do\(\))").unwrap();
    let re2 = Regex::new(r"(don't\(\))").unwrap();

    let mut results_do: Vec<usize> = vec![];
    let mut results_dont: Vec<usize> = vec![];
    for i in re.captures_iter(input).map(|c| c.get(1).unwrap().start()) {
        results_do.push(i);
    }
    for i in re2.captures_iter(input).map(|c| c.get(1).unwrap().start()) {
        results_dont.push(i);
    }

    let mut mul_enabled = true;

    while results.len() > 0 {
        let first = vec![results.first().unwrap_or(&(usize::MAX, -1, -1)).0, *results_do.first().unwrap_or(&usize::MAX), *results_dont.first().unwrap_or(&usize::MAX)];

        let min = first.iter().min().unwrap();
        let position = first.iter().position(|x| x == min).unwrap();

        if position == 0 {
            if mul_enabled {
                total += results[0].1*results[0].2;
            }
            results.remove(0);
        } else if position == 1 {
            mul_enabled = true;
            results_do.remove(0);
        } else if position == 2 {
            mul_enabled = false;
            results_dont.remove(0);
        }
    } 

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn it_works_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part2(input);
        assert_eq!(result, 48);
    }
}