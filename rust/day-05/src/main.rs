use std::{
    collections::HashMap,
    io::{self, Read},
    usize,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Day 5, problem 1: {}", problem_1(&input));
    println!("Day 5, problem 2: {}", problem_2(&input));
}

fn problem_1(input: &String) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_rules(&parts[0]);
    let updates = parse_updates(&parts[1]);
    updates
        .iter()
        .filter(|update| valid_update(&rules, &update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn problem_2(input: &String) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_rules(&parts[0]);
    let updates = parse_updates(&parts[1]);
    updates
        .iter()
        .filter(|update| !valid_update(&rules, &update))
        .map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| match rules.get(a) {
                Some(right) => match right.contains(b) {
                    true => return std::cmp::Ordering::Less,
                    false => return std::cmp::Ordering::Greater,
                },
                None => std::cmp::Ordering::Greater,
            });
            sorted[sorted.len() / 2]
        })
        .sum()
}

fn parse_rules(input: &str) -> HashMap<usize, Vec<usize>> {
    input.lines().collect::<Vec<&str>>().iter().fold(
        HashMap::new(),
        |mut acc: HashMap<usize, Vec<usize>>, x| {
            let [left, right]: [usize; 2] = x
                .split("|")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            acc.entry(left)
                .and_modify(|rule| rule.push(right))
                .or_insert(vec![right]);
            acc
        },
    )
}

fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
fn valid_update(rules: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    let mut is_valid = true;
    let empty = vec![];
    for i in 0..update.len() {
        let right = &update[i + 1..update.len()];
        let rule = rules.get(&update[i]).unwrap_or(&empty);
        if !(right.iter().all(|x| rule.contains(x))) {
            is_valid = false;
            break;
        }
    }
    is_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.to_string());
        assert_eq!(result, 143);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.to_string());
        assert_eq!(result, 123);
    }
}
