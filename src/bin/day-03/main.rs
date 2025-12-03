fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = order_lists(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn order_lists(input: &str) -> (Vec<Vec<i64>>) {
    let mut result: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let splitted= line.split("").collect::<Vec<&str>>();
        result.push(splitted.iter().filter(|&&x| !x.is_empty()).map(|&x|{ x.parse().unwrap()}).collect());
    }
    result
}

fn part1(input: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;

    for bank in input {
        let mut position_decade = bank.iter().position(|x| x == bank.iter().max().unwrap()).unwrap();
        if position_decade == bank.iter().count()-1 {
            position_decade=bank[..position_decade].iter().position(|x| x == bank[..position_decade].iter().max().unwrap()).unwrap();
        }

        let position_unit = bank[position_decade+1..].iter().position(|x| x == bank[position_decade+1..].iter().max().unwrap()).unwrap();

        let mut full_number = bank[position_decade].to_string();
        full_number.push_str(&(bank[position_decade + 1 + position_unit].to_string()));

        total += full_number.parse::<i64>().unwrap();
    }

    total
}

fn part2(input: Vec<Vec<i64>>) -> i128 {
    let mut total: i128 = 0;

    for bank in input {
        let mut full_number:String = "".to_string();
        let mut larger_count = 0;
        for i in 0..12{
            let mut position_decade = bank[larger_count..].iter().position(|x| x == bank[larger_count..].iter().max().unwrap()).unwrap();
            if position_decade >= bank.iter().count()-1-12 {
                position_decade=bank[..position_decade].iter().position(|x| x == bank[..position_decade].iter().max().unwrap()).unwrap();
            }

            dbg!(bank[larger_count + position_decade]);

            full_number.push_str(&(bank[larger_count + position_decade].to_string()));
            larger_count = position_decade + 1;
        }

        total += full_number.parse::<i128>().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let order = order_lists(input);
        let result = part1(order);
        assert_eq!(result, 357);
    }

    #[test]
    fn it_works_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let order = order_lists(input);
        let result = part2(order);
        assert_eq!(result, 3121910778619);
    }
}