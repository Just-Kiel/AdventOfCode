use core::hash;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let ordered_lists = parse_input(input);
    let result1 = part1(ordered_lists.clone(), 1000);
    dbg!(result1);
    let ordered_lists2 = parse_input(input);
    let part2 = part2(ordered_lists2);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    let mut positions = Vec::new();
    
    for (i_line, line) in input.lines().enumerate() {
        let temp: Vec<i32> = line.split(",").flat_map(|x| x.parse::<i32>()).collect();
        positions.push((temp[0], temp[1], temp[2]));
    }

    positions
}

fn calculate_distance(point1: (i32, i32, i32), point2: (i32, i32, i32)) -> i32 {
    let square = (point1.0 - point2.0).pow(2) + (point1.1 - point2.1).pow(2) + (point1.2 - point2.2).pow(2);
    square.isqrt() as i32
}

fn part1(input: Vec<(i32, i32, i32)>, max: i32) -> i64 {
    let mut total = 0;

    let mut distances: Vec<(i32, (i32, i32, i32), (i32, i32, i32))> = Vec::new();
    for (index, position) in input.iter().enumerate() {
        let mut testable_positions = input.clone();
        testable_positions.remove(index);
        for tested_position in testable_positions {
            if !(distances.iter().find(|&x| *x == (calculate_distance(*position, tested_position), tested_position, *position)).is_some()){
                distances.push((calculate_distance(*position, tested_position), *position, tested_position));
            }
            // hashmap.entry(*position).and_modify(|x| x.push((tested_position, calculate_distance(*position, tested_position)))).or_insert(vec![(tested_position, calculate_distance(*position, tested_position))]);
        }
    }
    distances.sort();
    
    let mut hashmap: HashMap<(i32, i32, i32), Vec<(i32, (i32, i32, i32), (i32, i32, i32))>> = HashMap::new();
    for i in 0..max {
        let current_edge = distances.get(i as usize).unwrap();
        if hashmap.iter().find(|&x| *x.0 == current_edge.1).is_some() {
            hashmap.entry(current_edge.1).and_modify(|x| x.push(*current_edge)).or_insert(vec![*current_edge]);
        } else if hashmap.iter().find(|&x| *x.0 == current_edge.2).is_some() {
            hashmap.entry(current_edge.2).and_modify(|x| x.push(*current_edge)).or_insert(vec![*current_edge]);
        } else {
            hashmap.entry(current_edge.1).and_modify(|x| x.push(*current_edge)).or_insert(vec![*current_edge]);
        }
    }

    let mut sizes = Vec::new();
    for pouet in hashmap.keys() {
        sizes.push(hashmap.get(pouet).unwrap().iter().count());
    }
    sizes.sort();
    sizes.reverse();

    dbg!(&sizes);

    total = sizes[0] * sizes[1] * sizes[2];

    total as i64
}

fn part2(input: Vec<(i32, i32, i32)>) -> i64 {
    let mut total = 0;

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let order = parse_input(input);
        let result = part1(order, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn it_works_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let order = parse_input(input);
        let result = part2(order);
        assert_eq!(result, 40);
    }
}