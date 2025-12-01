fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = order_lists(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn order_lists(input: &str) -> (Vec<&str>, Vec<i32>) {
    let mut left: Vec<&str> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let splitted= line.split_at(1);
        left.push(splitted.0);
        right.push(splitted.1.parse().unwrap());
    }

    (left, right)
}

fn part1(input: (Vec<&str>, Vec<i32>)) -> i32 {
    let start = 50;
    let mut current = start;
    let mut total = 0;

    let (directions, count) = input;

    for (index, direction) in directions.iter().enumerate(){
        if *direction == "L"{
            let mut step = count[index];
            while step > 0 {
                current -= 1;
                if current < 0 {
                    current = 99;
                }
                step -= 1;
            }
        } else {  
            let mut step = count[index];
            while step > 0 {
                current += 1;
                if current > 99 {
                    current = 0;
                }
                step -= 1;
            }         
        }

        if current == 0{
            total+=1;
        }
    }

    total
}

fn part2(input: (Vec<&str>, Vec<i32>)) -> i32 {
    let start = 50;
    let mut current = start;
    let mut total = 0;

    let (directions, count) = input;

    for (index, direction) in directions.iter().enumerate(){
        if *direction == "L"{
            let mut step = count[index];
            while step > 0 {
                current -= 1;
                if current < 0 {
                    current = 99;
                }
                if current == 0{
                    total+=1;
                }
                step -= 1;
            }
        } else {  
            let mut step = count[index];
            while step > 0 {
                current += 1;
                if current > 99 {
                    current = 0;
                }
                if current == 0{
                    total+=1;
                }
                step -= 1;
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
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let order = order_lists(input);
        let result = part1(order);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let order = order_lists(input);
        let result = part2(order);
        assert_eq!(result, 6);
    }
}