fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = parse_input(input);
    let part1 = part1(ordered_lists.clone());
    dbg!(part1);
    let part2 = part2(ordered_lists);
    dbg!(part2);
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut freshRanges = Vec::new();
    let mut ingredients = Vec::new();

    let mut parsing_ranges = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let splitted= line.split("-").collect::<Vec<&str>>();
            freshRanges.push((splitted[0].parse().unwrap(), splitted[1].parse().unwrap()));
            continue;
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    (freshRanges, ingredients)
}

fn part1(input: (Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let mut total = 0;

    let freshRanges = input.0;
    let ingredients = input.1;

    for ingredient in ingredients {
        for range in &freshRanges {
            if ingredient >= range.0 && ingredient <= range.1 {
                total += 1;
                break;
            }
        }
    }

    total
}

fn part2(input: (Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let freshRanges = input.0;

    let mut fresh: Vec<(i64, i64)> = Vec::new();
    for range in &freshRanges {
        if fresh.is_empty() {
            fresh.push(*range);
            continue;
        }

        let mut is_overlapping = false;
        for i in 0..fresh.len() {
            let existing_range = fresh[i];
            if range.0 <= existing_range.1 && range.1 >= existing_range.0 {
                let new_start = std::cmp::min(range.0, existing_range.0);
                let new_end = std::cmp::max(range.1, existing_range.1);
                fresh[i] = (new_start, new_end);
                is_overlapping = true;
                break;
            }
        }
        if !is_overlapping {
            fresh.push(*range);
        }
        
    }

    let mut is_not_finished =true;
    while is_not_finished {
        is_not_finished = false;
        let mut i = 0;
        while i < fresh.len() {
            let mut j = i + 1;
            while j < fresh.len() {
                let range1 = fresh[i];
                let range2 = fresh[j];
                if range1.0 <= range2.1 && range1.1 >= range2.0 {
                    let new_start = std::cmp::min(range1.0, range2.0);
                    let new_end = std::cmp::max(range1.1, range2.1);
                    fresh[i] = (new_start, new_end);
                    fresh.remove(j);
                    is_not_finished = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    let total = fresh.iter().map(|range| range.1 - range.0 +1).sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let order = parse_input(input);
        let result = part1(order);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let order = parse_input(input);
        let result = part2(order);
        assert_eq!(result, 14);
    }
}