advent_of_code::solution!(11);

fn manhatten_distance(p1: (usize, usize), p2: (usize, usize)) -> u32 {
    return (p2.0.abs_diff(p1.0) + p2.1.abs_diff(p1.1)) as u32;
}

fn manhatten_distance_u64(p1: (usize, usize), p2: (usize, usize)) -> u64 {
    return (p2.0.abs_diff(p1.0) + p2.1.abs_diff(p1.1)) as u64;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut points: Vec<(usize, usize)> = Vec::new();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match grid[row][col] {
                '#' => {
                    points.push((col, row))
                }
                _ => {}
            }
        }
    }

    let empty_rows: Vec<bool> = grid.iter()
    .map(|l| l.iter().all(|c| *c == '.'))
    .collect();

    let mut empty_cols: Vec<bool> = Vec::with_capacity(grid.len());

    for col in 0..grid[0].len() {
        let mut all = true;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                all = false;
            }
        }
        empty_cols.push(all);
    }

    points = points.iter()
    .map(|(x, y)| {
        let empty_rows_before = empty_rows.iter()
        .enumerate()
        .filter(|(i, &empty)| {
            i < y && empty
        }).count();

        let empty_cols_before = empty_cols.iter()
        .enumerate()
        .filter(|(i, &empty)| {
            i < x && empty
        }).count();

        (x + empty_cols_before, y + empty_rows_before)

    })
    .collect();

    let mut distance = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            distance += manhatten_distance(p1, p2);
        }
    }

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points: Vec<(usize, usize)> = Vec::new();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match grid[row][col] {
                '#' => {
                    points.push((col, row))
                }
                _ => {}
            }
        }
    }

    let empty_rows: Vec<bool> = grid.iter()
    .map(|l| l.iter().all(|c| *c == '.'))
    .collect();

    let mut empty_cols: Vec<bool> = Vec::with_capacity(grid.len());

    for col in 0..grid[0].len() {
        let mut all = true;
        for row in 0..grid.len() {
            if grid[row][col] == '#' {
                all = false;
            }
        }
        empty_cols.push(all);
    }

    points = points.iter()
    .map(|(x, y)| {
        let empty_rows_before = empty_rows.iter()
        .enumerate()
        .filter(|(i, &empty)| {
            i < y && empty
        }).count();

        let empty_cols_before = empty_cols.iter()
        .enumerate()
        .filter(|(i, &empty)| {
            i < x && empty
        }).count();

        (x + empty_cols_before * 999999, y + empty_rows_before * 999999)

    })
    .collect();

    let mut distance = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            distance += manhatten_distance_u64(p1, p2);
        }
    }

    Some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
