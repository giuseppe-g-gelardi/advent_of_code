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

    safe_reports
}

fn part_two(input: &String) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            // Check if the report is safe using `problem_dampener`
            problem_dampener(&report).map(|(is_safe, _)| is_safe)
        })
        .filter(|&is_safe| is_safe)
        .count() as i32
}

fn problem_dampener(sequence: &[i32]) -> Option<(bool, Option<usize>)> {
    // Check if the sequence is already safe
    if is_sequence_safe(sequence) {
        return Some((true, None)); // Safe without removing any level
    }

    // Try removing each level (except the first and last) to see if it makes the sequence safe
    for i in 1..sequence.len() - 1 {
        let mut modified_sequence = sequence.to_vec();
        modified_sequence.remove(i); // Remove the level at index `i`
        if is_sequence_safe(&modified_sequence) {
            return Some((true, Some(i))); // Safe by removing the level at index `i`
        }
    }

    Some((false, None)) // Unsafe regardless of which level is removed
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

    let part_one_result = part_one(&input);
    let part_two_result = part_two(&input);

    println!("Part One: {}", part_one_result);
    println!("Part Two: {}", part_two_result);

    Ok(())
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
    fn test_part_one_puzzle_input() {
        // verify with puzzle input
        let input = read_input("./src/day_2/input.txt").unwrap();
        let result = part_one(&input);
        assert_eq!(result, 314);
    }

    #[test]
    fn test_part_two() {
        let input = test_input();
        let result = part_two(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two_puzzle_input() {
        // verify with puzzle input
        let input = read_input("./src/day_2/input.txt").unwrap();
        let result = part_two(&input);
        assert_eq!(result, 0);
    }
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
