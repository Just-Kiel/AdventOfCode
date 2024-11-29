fn main() {
    let input = include_str!("./input.txt");
    let part1 = part1(input);
    dbg!(part1);
    let part2 = part2(input);
    dbg!(part2);
}

fn part1(input: &str) -> String {
    let mut current_position = (0, 0);
    let mut houses_delivered = vec![(0, 0)];

    for m in input.chars() {
        match m {
            '<' => current_position.0 += 1,
            '>' => current_position.0 -= 1,
            '^' => current_position.1 += 1,
            'v' => current_position.1 -= 1,
            _ => (),
        }
        if !houses_delivered.contains(&current_position) {
            houses_delivered.push(current_position);
        }
    }
    houses_delivered.len().to_string()
}

fn part2(input: &str) -> String {
    let mut current_position = vec![(0, 0), (0,0)];
    let mut houses_delivered = vec![(0, 0)];

    for (i, m) in input.chars().enumerate() {
        match m {
            '<' => current_position[i%2].0 += 1,
            '>' => current_position[i%2].0 -= 1,
            '^' => current_position[i%2].1 += 1,
            'v' => current_position[i%2].1 -= 1,
            _ => (),
        }

        for pos in current_position.iter().copied() {
            if !houses_delivered.contains(&pos) {
                houses_delivered.push(pos);
            }
        }
    }

    houses_delivered.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "^>v<";
        let result = part1(input);
        assert_eq!(result, "4".to_string());
    }

    #[test]
    fn it_works_part2() {
        let input = "^>v<";
        let result = part2(input);
        assert_eq!(result, "3".to_string());
    }
}