use std::io;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("Day 4, problem 1: {}", problem_1(&lines));
    println!("Day 4, problem 2: {}", problem_2(&lines));
}

fn problem_1(input: &Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let pattern = [
        // (Char to find, Y move from current point, X move from current point)
        [('X', 0, 0), ('M', 0, -1), ('A', 0, -2), ('S', 0, -3)], // left
        [('X', 0, 0), ('M', -1, -1), ('A', -2, -2), ('S', -3, -3)], // left up
        [('X', 0, 0), ('M', -1, 0), ('A', -2, 0), ('S', -3, 0)], // up
        [('X', 0, 0), ('M', -1, 1), ('A', -2, 2), ('S', -3, 3)], // right up
        [('X', 0, 0), ('M', 0, 1), ('A', 0, 2), ('S', 0, 3)],    // right
        [('X', 0, 0), ('M', 1, 1), ('A', 2, 2), ('S', 3, 3)],    // right down
        [('X', 0, 0), ('M', 1, 0), ('A', 2, 0), ('S', 3, 0)],    // down
        [('X', 0, 0), ('M', 1, -1), ('A', 2, -2), ('S', 3, -3)], // left down
    ]
    .to_vec()
    .iter()
    .map(|a| a.to_vec())
    .collect();

    search(grid, &pattern)
}

fn problem_2(input: &Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let pattern = [
        // M.S
        // .A.
        // M.S
        [
            ('A', 0, 0),
            ('M', -1, -1),
            ('S', -1, 1),
            ('M', 1, -1),
            ('S', 1, 1),
        ],
        // S.S
        // .A.
        // M.M
        [
            ('A', 0, 0),
            ('S', -1, -1),
            ('S', -1, 1),
            ('M', 1, -1),
            ('M', 1, 1),
        ],
        // S.M
        // .A.
        // S.M
        [
            ('A', 0, 0),
            ('S', -1, -1),
            ('M', -1, 1),
            ('S', 1, -1),
            ('M', 1, 1),
        ],
        // M.M
        // .A.
        // S.S
        [
            ('A', 0, 0),
            ('M', -1, -1),
            ('M', -1, 1),
            ('S', 1, -1),
            ('S', 1, 1),
        ],
    ]
    .to_vec()
    .iter()
    .map(|a| a.to_vec())
    .collect();

    search(grid, &pattern)
}

fn search(grid: Vec<Vec<char>>, pattern: &Vec<Vec<(char, isize, isize)>>) -> usize {
    let x_len = grid[0].len();
    let y_len = grid.len();
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            for p in pattern {
                let mut found = true;
                for (char_to_find, y_move, x_move) in p {
                    let new_y = y as isize + y_move;
                    let new_x = x as isize + x_move;
                    if !(new_x >= 0
                        && new_x < x_len as isize
                        && new_y >= 0
                        && new_y < y_len as isize
                        && &grid[new_y as usize][new_x as usize] == char_to_find)
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn problem_1_works() {
        let result = problem_1(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 18);
    }

    #[test]
    fn problem_2_works() {
        let result = problem_2(&TEST_INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(result, 9);
    }
}
