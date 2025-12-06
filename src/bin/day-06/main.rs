fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = parse_input(input);
    let result1 = part1(ordered_lists.clone());
    dbg!(result1);
    let ordered_lists2 = parse_input_2(input);
    let part2 = part1(ordered_lists2);
    dbg!(part2);
}

fn parse_input(input: &str) -> (Vec<(Vec<i64>, &str)>) {
    let mut total = Vec::new();
    
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            let count = line.split_whitespace().count();

            for _ in 0..count {
                total.push((Vec::new(), ""));
            }
        }

        line.split_whitespace().enumerate().for_each(|(index, num_str)| {
            if let Ok(num) = num_str.parse::<i64>() {
                total[index].0.push(num);
            } else {
                total[index].1 = num_str;
            }
        });
    }

    total
}

fn parse_input_2(input: &str) -> (Vec<(Vec<i64>, &str)>) {
    let mut length_line = 0;
    let mut total = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            length_line = line.chars().count();

            let count = line.split_whitespace().count();

            for _ in 0..count {
                total.push((Vec::new(), ""));
            }
            break;
        } 
    }

    let mut temp: Vec<char> = Vec::new();
    let total_size = total.iter().count();
    let mut current_in_total = total_size-1;

    for index in 0..length_line {
        temp.clear();
        for (i, line) in input.lines().enumerate() {
            if i == input.lines().count()-1{
                line.split_whitespace().enumerate().for_each(|(index, num_str)| {
                    if let Ok(num) = num_str.parse::<i64>() {
                    } else {
                        total[index].1 = num_str;
                    }
                });
                continue;
            } else {
                temp.push(line.chars().nth(length_line-1-index).unwrap());
            }
        }

        if !temp.iter().collect::<String>().trim().is_empty(){
            total[current_in_total].0.push(temp.iter().collect::<String>().trim().parse::<i64>().unwrap());
        } else {
            current_in_total -=1;
        }
    }

    total
}

fn part1(input: (Vec<(Vec<i64>, &str)>)) -> i64 {
    let mut total = 0;

    for (numbers, operation) in input {
        let result = match operation {
            "+" => numbers.iter().sum(),
            "*" => numbers.iter().product(),
            _ => 0,
        };
        total += result;
    }

    total
}

fn part2(input: (Vec<(Vec<i64>, &str)>)) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let order = parse_input(input);
        let result = part1(order);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn it_works_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let order = parse_input_2(input);
        let result = part1(order);
        assert_eq!(result, 3263827);
    }
}