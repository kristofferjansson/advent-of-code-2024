use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("Day 3, problem 1: {}", problem_1(&input));
    println!("Day 3, problem 2: {}", problem_2(&input));
}

fn problem_1(input: &String) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut memory = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        memory.push(a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap());
    }
    memory.iter().sum()
}

fn problem_2(input: &String) -> usize {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled: bool = true;
    let mut memory = vec![];
    for caps in re.captures_iter(input) {
        let op = &caps[0];
        match op {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => match enabled {
                true => {
                    let a = &caps[1];
                    let b = &caps[2];
                    memory.push(a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap());
                }
                false => {}
            },
        }
    }
    memory.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT_1.to_string());
        assert_eq!(result, 161);
    }

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT_2.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 48);
    }
}
