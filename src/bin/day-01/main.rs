fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = order_lists(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn order_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        left.push(splitted.first().unwrap().parse().unwrap());
        right.push(splitted.last().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();
    (left, right)
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = input;

    let mut total = 0;

    for (i, l) in left.iter().enumerate() {
        total += i32::abs(l-right[i]);
    }

    total
}

fn part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut total = 0;
    let (left, right) = input;
    for e in left {
        let mut occurrence = 0;
        for r in right.iter() {
            if *r == e {
                occurrence += 1;
            }
        }
        total += occurrence * e;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let order = order_lists(input);
        let result = part1(order);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_works_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let order = order_lists(input);
        let result = part2(order);
        assert_eq!(result, 31);
    }
}