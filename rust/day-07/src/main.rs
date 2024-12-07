use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 7, problem 1: {}", problem_1(&lines));
    println!("Day 7, problem 2: {}", problem_2(&lines));
}

enum Operation {
    Mul,
    Add,
    Concat,
}

fn problem_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(parse_equation)
        .filter(|(test_val, numbers)| {
            valid_equation(test_val, numbers, vec![Operation::Add, Operation::Mul])
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

fn problem_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(parse_equation)
        .filter(|(test_val, numbers)| {
            valid_equation(
                test_val,
                numbers,
                vec![Operation::Add, Operation::Mul, Operation::Concat],
            )
        })
        .map(|(test_val, _)| test_val)
        .sum()
}

fn parse_equation(line: &String) -> (usize, Vec<usize>) {
    let parts: Vec<&str> = line.split(":").collect();
    let test_val: usize = parts[0].trim().parse::<usize>().unwrap();
    let numbers: Vec<usize> = parts[1]
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    (test_val, numbers)
}

fn valid_equation(test_val: &usize, numbers: &Vec<usize>, ops: Vec<Operation>) -> bool {
    let mut perms: Vec<usize> = vec![numbers[0]];
    for i in 1..numbers.len() {
        let mut updated: Vec<usize> = vec![];
        for perm in perms {
            if &perm > test_val {
                continue;
            }
            for op in &ops {
                match op {
                    Operation::Mul => updated.push(perm * numbers[i]),
                    Operation::Add => updated.push(perm + numbers[i]),
                    Operation::Concat => updated.push(
                        (perm.to_string() + &numbers[i].to_string())
                            .parse::<usize>()
                            .unwrap(),
                    ),
                }
            }
        }
        perms = updated;
    }
    perms.contains(&test_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 3749);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 11387);
    }
}
