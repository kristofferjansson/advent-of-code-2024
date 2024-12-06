use std::{io, isize};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 6, problem 1: {}", problem_1(&lines));
    println!("Day 6, problem 2: {}", problem_2(&lines));
}

#[derive(Debug, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    current_position: Position,
    direction: Direction,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Obstacle,
    Empty,
    Visited,
}

fn problem_1(input: &Vec<String>) -> usize {
    let (guard, mut grid) = parse_input(input);
    grid = patrol(grid, guard).1;
    grid.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|tile| tile == &&Tile::Visited).count()
    }) + 1
}

fn problem_2(input: &Vec<String>) -> usize {
    let (guard, grid) = parse_input(input);

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Tile::Empty {
                let mut new_grid = grid.clone();
                let new_guard = guard.clone();
                new_grid[y][x] = Tile::Obstacle;
                let is_looping = patrol(new_grid, new_guard).0;
                if is_looping {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse_input(input: &Vec<String>) -> (Guard, Vec<Vec<Tile>>) {
    let mut guard = Guard {
        current_position: Position { x: 0, y: 0 },
        direction: Direction::Up,
    };
    let grid = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard = Guard {
                            current_position: Position {
                                x: x as isize,
                                y: y as isize,
                            },
                            direction: Direction::Up,
                        };
                        Tile::Visited
                    }
                    _ => panic!("Invalid character in input"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    (guard, grid)
}

fn patrol(mut grid: Vec<Vec<Tile>>, mut guard: Guard) -> (bool, Vec<Vec<Tile>>) {
    let y_len = grid.len() as isize;
    let x_len = grid[0].len() as isize;
    let mut moves = 0;
    loop {
        let new_pos: Position = match guard.direction {
            Direction::Up => Position {
                x: guard.current_position.x,
                y: guard.current_position.y - 1,
            },
            Direction::Right => Position {
                x: guard.current_position.x + 1,
                y: guard.current_position.y,
            },
            Direction::Down => Position {
                x: guard.current_position.x,
                y: guard.current_position.y + 1,
            },
            Direction::Left => Position {
                x: guard.current_position.x - 1,
                y: guard.current_position.y,
            },
        };

        if !(new_pos.y >= 0 && new_pos.y < y_len && new_pos.x >= 0 && new_pos.x < x_len) {
            break;
        }

        match grid[new_pos.y as usize][new_pos.x as usize] {
            Tile::Obstacle => match guard.direction {
                Direction::Up => guard.direction = Direction::Right,
                Direction::Right => guard.direction = Direction::Down,
                Direction::Down => guard.direction = Direction::Left,
                Direction::Left => guard.direction = Direction::Up,
            },
            Tile::Empty | Tile::Visited => {
                grid[guard.current_position.y as usize][guard.current_position.x as usize] =
                    Tile::Visited;
                guard.current_position = new_pos;
            }
        }

        moves += 1;

        // ugly loop detection
        if moves > 10000 {
            return (true, grid);
        }
    }

    (false, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 41);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 6);
    }
}
