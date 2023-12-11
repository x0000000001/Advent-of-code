mod helpers;

pub use helpers::Solution;

type InputType = Vec<Vec<bool>>;
type Pos = (usize, usize);

fn empty_columns_and_lines(grid: &InputType) -> (Vec<usize>, Vec<usize>) {
    (
        (0..grid[0].len())
            .filter(|&j| (0..grid.len()).all(|i| !grid[i][j]))
            .collect(),
        (0..grid.len())
            .filter(|&i| (0..grid[0].len()).all(|j| !grid[i][j]))
            .collect(),
    )
}

fn distance(
    p0: &Pos,
    p1: &Pos,
    empty_columns: &[usize],
    empty_lines: &[usize],
    bonus: usize,
) -> usize {
    let x_min = p0.0.min(p1.0);
    let x_max = p0.0.max(p1.0);
    let y_min = p0.1.min(p1.1);
    let y_max = p0.1.max(p1.1);

    (x_max - x_min + y_max - y_min)
        + (empty_columns
            .iter()
            .filter(|&&y| y > y_min && y < y_max)
            .count()
            * (bonus - 1))
        + (empty_lines
            .iter()
            .filter(|&&x| x > x_min && x < x_max)
            .count()
            * (bonus - 1))
}

fn sum_of_distances(grid: &InputType, bonus: usize) -> usize {
    let (empty_columns, empty_lines) = empty_columns_and_lines(grid);

    let mut pos = vec![];

    for (i, line) in grid.iter().enumerate() {
        for (j, b) in line.iter().enumerate() {
            if *b {
                pos.push((i, j));
            }
        }
    }

    let mut sum = 0;

    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            sum += distance(&pos[i], &pos[j], &empty_columns, &empty_lines, bonus)
        }
    }

    sum
}

pub fn part1(s: String) -> Solution {
    let grid = parse(s);
    Solution::from(sum_of_distances(&grid, 2) as u64)
}

pub fn part2(s: String) -> Solution {
    let grid = parse(s);
    Solution::from(sum_of_distances(&grid, 1000000) as u64)
}

fn parse(s: String) -> InputType {
    s.lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}
