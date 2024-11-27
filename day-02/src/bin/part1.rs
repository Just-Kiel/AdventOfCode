use std::vec;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // split lines
    let mut wrapping_paper = 0;
    let lines = input.lines();
    for line in lines {
        let dimensions: Vec<&str> = line.split("x").collect();
        let l: i32 = dimensions[0].parse().unwrap();
        let w: i32 = dimensions[1].parse().unwrap();
        let h: i32 = dimensions[2].parse().unwrap();

        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let mut total = 2*lw + 2*wh + 2*hl;

        let areas = vec![lw, wh, hl];
        
        total += areas.iter().min().unwrap();
        wrapping_paper += total;
    }
    wrapping_paper.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2x3x4";
        let result = part1(input);
        assert_eq!(result, "58".to_string());
    }
}