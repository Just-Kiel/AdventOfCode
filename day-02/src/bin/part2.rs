fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut ribbon = 0;
    let lines = input.lines();
    for line in lines {
        let mut dimensions: Vec<i32> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();

        let bow: i32 = dimensions.iter().copied().reduce(|a:i32, b:i32| a*b).unwrap();
        
        let max= dimensions.iter().max().unwrap();
        dimensions.remove(dimensions.iter().position(|x| x == max).unwrap());

        let wrap = dimensions[0]*2 + dimensions[1]*2;

        ribbon += bow + wrap;
    }
    ribbon.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2x3x4";
        let result = part2(input);
        assert_eq!(result, "34".to_string());
    }
}