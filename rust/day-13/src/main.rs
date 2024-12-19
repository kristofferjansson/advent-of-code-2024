use itertools::{Chunk, Itertools};
use regex::Regex;
use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 13, problem 1: {}", problem_1(&lines));
    println!("Day 13, problem 2: {}", problem_2(&lines));
}

struct Machine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize: Coordinates,
}
struct Coordinates {
    x: f64,
    y: f64,
}

impl Machine {
    fn determinant(&self) -> f64 {
        self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x
    }
    fn solve(&self) -> usize {
        if self.determinant() == 0.0 {
            // If determinant is 0, no unique solution exists
            return 0;
        }

        let x =
            (self.prize.x * self.button_b.y - self.prize.y * self.button_b.x) / self.determinant();
        let y =
            (self.button_a.x * self.prize.y - self.button_a.y * self.prize.x) / self.determinant();

        if x.fract() != 0.0 || y.fract() != 0.0 {
            return 0;
        }
        let res = (x as isize * 3) + y as isize;
        res as usize
    }
}

fn problem_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .chunks(4)
        .into_iter()
        .map(|chunk| extract_machine(chunk, Regex::new(r"X[+=](\d+).*Y[+=](\d+)").unwrap(), 0.0))
        .map(|machine| machine.solve())
        .sum()
}

fn problem_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            extract_machine(
                chunk,
                Regex::new(r"X[+=](\d+).*Y[+=](\d+)").unwrap(),
                10000000000000.0,
            )
        })
        .map(|machine| machine.solve())
        .sum()
}

fn extract_machine(
    chunk: Chunk<'_, std::slice::Iter<'_, String>>,
    re: Regex,
    add_to_prize: f64,
) -> Machine {
    match &chunk.collect::<Vec<&String>>()[..] {
        [a, b, prize, ..] => {
            let button_a = extract_coords(&re, a);
            let button_b = extract_coords(&re, b);
            let mut prize = extract_coords(&re, prize);
            prize.x += add_to_prize;
            prize.y += add_to_prize;
            return Machine {
                button_a,
                button_b,
                prize,
            };
        }
        _ => panic!(),
    }
}

fn extract_coords(re: &Regex, input: &String) -> Coordinates {
    let (x, y): (f64, f64) = re
        .captures(input)
        .and_then(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
        .unwrap_or((0.0, 0.0));
    Coordinates { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 480);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 875318608908);
    }
}
