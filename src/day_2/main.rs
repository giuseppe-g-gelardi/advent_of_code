use advent_of_code_2024::read_input;

type Report = Vec<i32>;

fn part_one(input: &String) -> i32 {
    let mut safe_reports = 0;
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Report>()
        })
        .collect::<Vec<Report>>();

    reports.iter().for_each(|report| {
        let is_safe = is_sequence_safe(report);
        if is_safe {
            safe_reports += 1;
        }
    });

    println!("Safe reports: {}", safe_reports);
    safe_reports
}

// BRUH its only day 2
fn is_sequence_safe(sequence: &[i32]) -> bool {
    if sequence.len() < 2 {
        return true; // Single element or empty sequence is trivially safe
    }

    let mut direction = 0; // 0 = undefined, 1 = increasing, -1 = decreasing

    for i in 0..sequence.len() - 1 {
        let diff = sequence[i + 1] - sequence[i];

        if diff == 0 || diff.abs() > 3 {
            return false; // Unsafe if difference is 0 or exceeds 3
        }

        let current_direction = diff.signum(); // Determine direction (-1, 0, or 1)

        if direction == 0 {
            direction = current_direction; // Set initial direction
        } else if direction != current_direction {
            return false; // Unsafe if direction changes
        }
    }
    true // Safe if all differences meet the criteria
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("./src/day_2/input.txt")?; // LMAO DELETE WHITE SPACE IN INPUT.TXT FILE
    // let input = test_input();
    part_one(&input);

    Ok(())
}

pub fn test_input() -> String {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .to_string();

    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = test_input();
        let result = part_one(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one_puzzle_input() { // verify with puzzle input
        let input = read_input("./src/day_2/input.txt").unwrap();
        let result = part_one(&input);
        assert_eq!(result, 314);
    }
}
