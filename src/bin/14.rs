advent_of_code::solution!(14);

use std::collections::{HashMap};

fn roll_north(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 'O' {
                continue;
            }

            let mut tmp_row = row;

            while tmp_row >= 1 && grid[tmp_row - 1][col] == '.' {
                grid[tmp_row - 1][col] = 'O';
                grid[tmp_row][col] = '.';
                tmp_row -= 1; 
            }

        }
    }
}

fn roll_west(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 'O' {
                continue;
            }

            let mut tmp_col = col;

            while tmp_col >= 1 && grid[row][tmp_col - 1] == '.' {
                grid[row][tmp_col - 1] = 'O';
                grid[row][tmp_col] = '.';
                tmp_col -= 1; 
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in (0..grid[0].len()).rev() {
            if grid[row][col] != 'O' {
                continue;
            }

            let mut tmp_col = col;

            while tmp_col < (grid[0].len() - 1) && grid[row][tmp_col + 1] == '.' {
                grid[row][tmp_col + 1] = 'O';
                grid[row][tmp_col] = '.';
                tmp_col += 1; 
            }
        }
    } 
}

fn roll_south(grid: &mut Vec<Vec<char>>) {
    for row in (0..grid.len()).rev() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 'O' {
                continue;
            }

            let mut tmp_row = row;

            while tmp_row < grid.len() - 1 && grid[tmp_row + 1][col] == '.' {
                grid[tmp_row + 1][col] = 'O';
                grid[tmp_row][col] = '.';
                tmp_row += 1; 
            }

        }
    }
}

fn spin_cycle(grid: &mut Vec<Vec<char>>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input.split('\n')
    .map(|split| split.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 'O' {
                continue;
            }

            let mut tmp_row = row;

            while tmp_row >= 1 && grid[tmp_row - 1][col] == '.' {
                grid[tmp_row - 1][col] = 'O';
                grid[tmp_row][col] = '.';
                tmp_row -= 1; 
            }

        }
    }

    let res: u32 = grid.iter()
    .enumerate()
    .map(|(i, row)| (row.iter().filter(|&c| *c == 'O').count() * (grid.len() - i)) as u32)
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input.split('\n')
    .map(|split| split.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut set: HashMap<String, i32> = HashMap::new();
    let mut loads: Vec<u32> = Vec::new();
    let mut index = 0;

    loop {
        let map = grid.iter().flat_map(|row| row.iter()).collect::<String>();

        if set.contains_key(&map) {
            let old_index = set.get(&map).unwrap();
            let cycle_len = index - old_index;
            return Some(loads[(((1000000000 - old_index)  % cycle_len) + old_index) as usize]);
        }

        let tmp: u32 = grid.iter()
        .enumerate()
        .map(|(i, row)| (row.iter().filter(|&c| *c == 'O').count() * (grid.len() - i)) as u32)
        .sum();

        loads.push(tmp);
            
        set.insert(map, index);
        index += 1;

        spin_cycle(&mut grid);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
