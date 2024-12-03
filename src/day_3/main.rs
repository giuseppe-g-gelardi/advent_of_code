use regex::Regex;

use advent_of_code_2024::read_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let input = test_input();
    let input = read_input("./src/day_3/input.txt")?; // LMAO DELETE WHITE SPACE IN INPUT.TXT FILE

    let result = part_one(&input);
    println!("Result: {}", result);

    Ok(())
}

fn part_one(input: &String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut number = Vec::new();

    for cap in re.captures_iter(&input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        number.push(a * b);
    }

    let result: i32 = number.iter().sum();

    result
}

fn test_input() -> String {
    return "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = test_input();
        let result = part_one(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_one_puzzle_input() {
        // verify with puzzle input
        let input = read_input("./src/day_3/input.txt").unwrap();
        let result = part_one(&input);
        assert_eq!(result, 173785482);
    }
}
