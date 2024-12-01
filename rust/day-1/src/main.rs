use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 1, problem 1: {}", problem_1(&lines));
    println!("Day 1, problem 2: {}", problem_2(&lines));
}

fn problem_1(input: &Vec<String>) -> usize {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    left.iter()
        .enumerate()
        .map(|(i, x)| x.abs_diff(right[i]))
        .sum()
}

fn problem_2(input: &Vec<String>) -> usize {
    let (left, right) = parse_input(input);
    left.iter()
        .map(|value| value * right.iter().filter(|&x| x == value).count())
        .sum()
}

fn parse_input(input: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    input
        .iter()
        .fold((Vec::<usize>::new(), Vec::<usize>::new()), |mut acc, f| {
            let parts: Vec<_> = f.split_whitespace().collect();
            let l: usize = parts[0].parse().unwrap();
            let r: usize = parts[1].parse().unwrap();
            acc.0.push(l);
            acc.1.push(r);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 11);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 31);
    }
}
