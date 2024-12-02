use std::vec;

fn main() {
    let input = include_str!("./input.txt");
    let levels = create_levels(input);
    let part1 = part1(levels.clone());
    dbg!(part1);
    let part2 = part2(levels);
    dbg!(part2);
}

fn create_levels(input: &str) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let splitted: Vec<&str> = line.split(" ").collect();
        let mut level: Vec<i32> = vec![];
        for s in splitted.iter() {
            level.push(s.parse().unwrap());
        }

        vec.push(level);
    }
    vec
}

fn is_safe(lvl: Vec<i32>) -> bool {
    let mut is_safe = true;
    let sign = (lvl[0] - lvl[1]) > 0;
    for (i, e) in lvl.iter().enumerate() {
        if i+1 == lvl.len() {
            break;
        }

        let diff = e - lvl[i+1];
        let current_sign = diff > 0;
        if current_sign != sign || i32::abs(diff) < 1 || i32::abs(diff) > 3 {
            is_safe = false;
            break;
        }
    }
    is_safe
}

fn part1(vec: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;

    for lvl in vec {
        let mut is_safe = true;
        let sign = (lvl[0] - lvl[1]) > 0;
        for (i, e) in lvl.iter().enumerate() {
            if i+1 == lvl.len() {
                break;
            }

            let diff = e - lvl[i+1];
            let current_sign = diff > 0;
            if current_sign != sign || i32::abs(diff) < 1 || i32::abs(diff) > 3 {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            total += 1;
        }
    }
    total
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    for level in input {
        let mut good = false;
        for j in 0..level.len() {
            let mut xs: Vec<i32> = level[..j].to_vec();
            xs.append(&mut level[j+1..].to_owned());
            if is_safe(xs) {
                good = true
            }
        }
        if good {
            total += 1
        }


        // MY METHOD DO NOT WORK
        // let mut differences: Vec<i32> = vec![];
        // for (i, e) in level.iter().enumerate() {
        //     if i+1 == level.len() {
        //         break;
        //     }

        //     let diff = e - level[i+1];
        //     differences.push(diff);
        // }

        // // count how many are over 3 or less than 1
        // let gaps = differences.iter().filter(|&n| i32::abs(*n) > 3 || i32::abs(*n) < 1).count();

        // // if more than 1 gap, we do not count it
        // if gaps > 1 {
        //     continue;
        // } else if gaps == 1 {
        //     // find the position and try to remove it
        //     let pos = differences.iter().position(|&n| i32::abs(n) > 3 || i32::abs(n) < 1).unwrap();

        //     let mut updated = level.clone();
        //     updated.remove(pos+1);

        //     if is_safe(updated) {
        //         total += 1;
        //     }
        // } else {
        //     if is_safe(level.clone()) {
        //         total += 1;
        //     } else {
        //         // count how many are positive and negative
        //         let positives = differences.iter().filter(|&n| *n > 0).count();
        //         if (differences.len() - 1 <= positives) || (differences.len() - 1 <= differences.len() - positives){
        //             total += 1;
        //         } else {
        //             dbg!(level);
        //         }
        //     }
        // }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let levels = create_levels(input);
        let result = part1(levels);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let levels = create_levels(input);
        let result = part2(levels);
        assert_eq!(result, 4);
    }
}