use regex::Regex;
use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 14, problem 1: {}", problem_1(&lines, 101, 103));
    println!("Day 14, problem 2: {}", problem_2(&lines, 101, 103));
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Coordinates,
    velocity: Coordinates,
}

#[derive(Debug, Clone)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Robot {
    fn parse(configuration: &String) -> Robot {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        let (px, py, vx, vy): (isize, isize, isize, isize) = re
            .captures(configuration)
            .and_then(|cap| {
                Some((
                    cap[1].parse().ok()?,
                    cap[2].parse().ok()?,
                    cap[3].parse().ok()?,
                    cap[4].parse().ok()?,
                ))
            })
            .unwrap_or((0, 0, 0, 0));

        Robot {
            pos: Coordinates { x: px, y: py },
            velocity: Coordinates { x: vx, y: vy },
        }
    }

    fn do_move(&self, width: isize, height: isize) -> Robot {
        let mut nx = self.pos.x + self.velocity.x;
        if nx > width - 1 {
            nx = nx - width;
        } else if nx < 0 {
            nx = nx + width;
        }

        let mut ny = self.pos.y + self.velocity.y;
        if ny > height - 1 {
            ny = ny - height;
        } else if ny < 0 {
            ny = ny + height;
        }

        Robot {
            pos: Coordinates { x: nx, y: ny },
            velocity: self.velocity.clone(),
        }
    }

    fn has_neighbours(&self, robots: &Vec<Robot>, num: usize) -> bool {
        for i in 0..num {
            if let Some(_) = robots
                .iter()
                .find(|r| r.pos.y == self.pos.y && r.pos.x == self.pos.x + i as isize)
            {
                continue;
            }
            return false;
        }
        true
    }
}

fn problem_1(input: &Vec<String>, width: isize, height: isize) -> usize {
    let mut robots: Vec<Robot> = input.iter().map(|line| Robot::parse(line)).collect();

    for _ in 0..100 {
        let new_robots: Vec<Robot> = robots
            .into_iter()
            .map(|robot| robot.do_move(width, height))
            .collect();

        robots = new_robots;
    }

    let middle_col = height / 2;
    let middle_row = width / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for y in 0..height {
        for x in 0..width {
            let num_robots = robots
                .iter()
                .filter(|r| r.pos.x == x && r.pos.y == y)
                .count();

            if num_robots > 0 {
                if x < middle_row && y < middle_col {
                    q1 += num_robots;
                } else if x > middle_row && y < middle_col {
                    q2 += num_robots;
                } else if x < middle_row && y > middle_col {
                    q3 += num_robots;
                } else if x > middle_row && y > middle_col {
                    q4 += num_robots;
                }
            }
        }
    }
    q1 * q2 * q3 * q4
}

fn problem_2(input: &Vec<String>, width: isize, height: isize) -> usize {
    let mut robots: Vec<Robot> = input.iter().map(|line| Robot::parse(line)).collect();

    let mut blinks = 0;
    let mut found = false;

    while !found {
        blinks += 1;
        let new_robots: Vec<Robot> = robots
            .into_iter()
            .map(|robot| robot.do_move(width, height))
            .collect();

        robots = new_robots;

        for robot in &robots {
            if robot.has_neighbours(&robots, 20) {
                found = true;
            }
        }
    }

    print_robots(&robots, width, height);

    blinks
}

fn print_robots(robots: &Vec<Robot>, width: isize, height: isize) {
    for y in 0..height {
        for x in 0..width {
            let num_robots = robots
                .iter()
                .filter(|r| r.pos.x == x && r.pos.y == y)
                .count();

            if num_robots > 0 {
                print!("{}", num_robots);
            } else {
                print!(".");
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect(), 11, 7);
        assert_eq!(result, 12);
    }
}
