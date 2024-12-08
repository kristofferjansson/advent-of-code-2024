use std::{
    collections::{HashMap, HashSet},
    io, vec,
};

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 8, problem 1: {}", problem_1(&lines));
    println!("Day 8, problem 2: {}", problem_2(&lines));
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Pos {
    y: isize,
    x: isize,
}

impl Pos {
    fn antinode_pairs(&self, other: &Pos) -> Vec<Pos> {
        vec![
            Pos {
                y: self.y + self.y - other.y,
                x: self.x + self.x - other.x,
            },
            Pos {
                y: other.y + other.y - self.y,
                x: other.x + other.x - self.x,
            },
        ]
    }

    fn antinodes<F>(&self, other: &Pos, predicate: F) -> Vec<Pos>
    where
        F: Fn(&Pos) -> bool,
    {
        let mut valid = vec![];
        let mut a = Pos {
            y: self.y + self.y - other.y,
            x: self.x + self.x - other.x,
        };
        while predicate(&a) {
            valid.push(a.clone());
            a = Pos {
                y: a.y + self.y - other.y,
                x: a.x + self.x - other.x,
            };
        }

        let mut b = Pos {
            y: other.y + other.y - self.y,
            x: other.x + other.x - self.x,
        };
        while predicate(&b) {
            valid.push(b.clone());
            b = Pos {
                y: b.y + other.y - self.y,
                x: b.x + other.x - self.x,
            };
        }
        valid
    }
}

fn problem_1(input: &Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut antinodes: HashSet<Pos> = HashSet::new();

    for positions in antennas(&grid).values() {
        for (a, b) in unique_pairs(&positions) {
            for antinode in a
                .antinode_pairs(b)
                .iter()
                .filter(|pos| in_grid(&grid, pos))
                .collect::<Vec<&Pos>>()
            {
                antinodes.insert(antinode.clone());
            }
        }
    }
    antinodes.len()
}

fn problem_2(input: &Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut antinodes: HashSet<Pos> = HashSet::new();

    for positions in antennas(&grid).values() {
        // All antennas are antinodes
        for pos in positions {
            antinodes.insert(pos.clone());
        }

        for (a, b) in unique_pairs(positions) {
            for antinode in a
                .antinodes(b, |pos: &Pos| in_grid(&grid, &pos))
                .iter()
                .collect::<Vec<&Pos>>()
            {
                antinodes.insert(antinode.clone());
            }
        }
    }
    antinodes.len()
}

fn antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Pos>> {
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                antennas
                    .entry(grid[y][x])
                    .and_modify(|e| {
                        e.push(Pos {
                            y: y as isize,
                            x: x as isize,
                        })
                    })
                    .or_insert(vec![Pos {
                        x: x as isize,
                        y: y as isize,
                    }]);
            }
        }
    }
    antennas
}

fn unique_pairs(positions: &[Pos]) -> Vec<(&Pos, &Pos)> {
    positions
        .iter()
        .enumerate()
        .flat_map(|(i, x)| positions.iter().skip(i + 1).map(move |y| (x, y)))
        .collect()
}

fn in_grid(grid: &Vec<Vec<char>>, pos: &Pos) -> bool {
    pos.y >= 0 && pos.y < grid.len() as isize && pos.x >= 0 && pos.x < grid[0].len() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 14);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 34);
    }
}
