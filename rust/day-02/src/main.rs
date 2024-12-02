use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 1, problem 1: {}", problem_1(&lines));
    println!("Day 1, problem 2: {}", problem_2(&lines));
}

fn problem_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .filter(valid_report)
        .collect::<Vec<Vec<usize>>>()
        .len()
}

fn problem_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .filter(valid_report_with_dampener)
        .collect::<Vec<Vec<usize>>>()
        .len()
}

fn valid_report_with_dampener(report: &Vec<usize>) -> bool {
    if valid_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if valid_report(&new_report) {
            return true;
        }
    }
    false
}

fn valid_report(report: &Vec<usize>) -> bool {
    is_valid(report, |a, b| a > b) || is_valid(report, |a, b| a < b)
}

fn is_valid<F>(report: &Vec<usize>, cmp: F) -> bool
where
    F: Fn(usize, usize) -> bool,
{
    for i in 0..report.len() - 1 {
        let diff = report[i].abs_diff(report[i + 1]);
        if cmp(report[i], report[i + 1]) && diff > 0 && diff <= 3 {
            continue;
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 2);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 4);
    }
}
