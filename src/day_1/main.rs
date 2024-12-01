use std::io::Error;

use advent_of_code_2024::read_input;

fn main() -> Result<(), std::io::Error> {
    let input = read_input("./src/day_1/input.txt")?;

    let sum_part_one = get_distance_part_one(&input)?;
    let sum_part_two = get_distance_part_two(&input)?;

    println!("sum part_one: {:?}", sum_part_one);
    println!("sum part_two: {:?}", sum_part_two);

    Ok(())
}

fn get_distance_part_two(input: &String) -> Result<i32, Error> {
    let mut sum = 0;
    let mut frequency = 0;

    let (left_list, right_list) = get_lists(input);
    let mut new_list: Vec<i32> = Vec::new();

    left_list.iter().for_each(|i| {
        right_list.iter().for_each(|j| {
            if i == j {
                frequency += 1;
            }

            new_list.push(i * frequency);
            frequency = 0;
        })
    });

    for i in 0..new_list.len() {
        sum += new_list[i];
    }

    Ok(sum)
}

fn get_distance_part_one(input: &String) -> Result<i32, Error> {
    let mut difference_list: Vec<i32> = Vec::new();
    let mut sum = 0;

    let (left_list, right_list) = get_lists(input);

    for i in 0..left_list.len() {
        difference_list.push(right_list[i] - left_list[i]);
    }

    for i in 0..difference_list.len() {
        sum += difference_list[i].abs();
    }

    Ok(sum)
}

pub fn test_input() -> String {
    let input = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    input.to_string()
}

pub fn get_lists(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<&str> = Vec::new();
    let mut right_list: Vec<&str> = Vec::new();

    // split the input into lines
    let lines: Vec<&str> = input.lines().collect();

    // split the lines into columns
    let columns: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    // split the columns into left and right
    for column in columns {
        left_list.push(column[0]);
        right_list.push(column[1]);
    }

    let mut left_list: Vec<i32> = left_list
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut right_list: Vec<i32> = right_list
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance_part_one_test_input() {
        let input = test_input();
        let expected = 11;

        assert_eq!(super::get_distance_part_one(&input).unwrap(), expected);
    }

    #[test]
    fn test_get_distance_part_two_test_input() {
        let input = test_input();
        let expected = 31;

        assert_eq!(super::get_distance_part_two(&input).unwrap(), expected);
    }
}
