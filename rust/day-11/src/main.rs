use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Day 11, problem 1: {}", problem_1(&input, 25));
    println!("Day 11, problem 2: {}", problem_2(&input, 75));
}

fn problem_1(input: &String, blinks: usize) -> usize {
    let stone_count: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    process_blinks(blinks, stone_count)
}

fn problem_2(input: &String, blinks: usize) -> usize {
    let stone_count: HashMap<usize, usize> = input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    process_blinks(blinks, stone_count)
}

fn process_blinks(iterations: usize, mut stone_count: HashMap<usize, usize>) -> usize {
    for _ in 0..iterations {
        let mut updated_stone_count: HashMap<usize, usize> = HashMap::default();
        for (stone, count) in stone_count.into_iter() {
            if stone == 0 {
                updated_stone_count
                    .entry(1)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            } else if stone.to_string().len() % 2 == 0 {
                let (a, b) = split_number(stone);
                updated_stone_count
                    .entry(a)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                updated_stone_count
                    .entry(b)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            } else {
                updated_stone_count
                    .entry(stone * 2024)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        }
        stone_count = updated_stone_count;
    }

    stone_count.values().sum()
}

fn split_number(number: usize) -> (usize, usize) {
    let digits = number.to_string();
    let len = digits.len();
    let mid = len / 2;

    let left: usize = digits[..mid].parse().unwrap();
    let right: usize = digits[mid..].parse().unwrap();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.to_string(), 25);
        assert_eq!(result, 55312);

        let result = problem_1(&TEST_INPUT.to_string(), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.to_string(), 25);
        assert_eq!(result, 55312);
    }
}
