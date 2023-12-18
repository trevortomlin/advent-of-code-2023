advent_of_code::solution!(16);

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use std::thread;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Electron {
    x: i32,
    y: i32,
    dir: Direction,
}

fn trace_beam(grid: &Vec<Vec<char>>, x: i32, y: i32, dir: Direction) -> Option<u32> {
    let mut visited: HashSet<Electron> = HashSet::new();
    let mut queue: VecDeque<Electron> = VecDeque::new();

    queue.push_back(Electron{x, y, dir});

    while !queue.is_empty() {
        let electron = queue.pop_front().unwrap();

        if electron.x < 0 || electron.x >= (grid[0].len() as i32) {
            continue;
        }
        if electron.y < 0 || electron.y >= (grid.len() as i32) {
            continue;
        }
        if visited.contains(&electron) {
            continue;
        }

        visited.insert(electron);

        let grid_cell = grid[electron.y as usize][electron.x as usize];

        match grid_cell {
            '.' => {
                match electron.dir {
                    Direction::Up => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y - 1,
                        dir: electron.dir,
                    }),
                    Direction::Down => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y + 1,
                        dir: electron.dir, 
                        
                    }),
                    Direction::Left => queue.push_back(Electron{
                        x: electron.x - 1,
                        y: electron.y,
                        dir: electron.dir, 
                        
                    }),
                    Direction::Right => queue.push_back(Electron{
                        x: electron.x + 1,
                        y: electron.y,
                        dir: electron.dir, 
                        
                    }),
                }
            }
            '\\' => {

                match electron.dir {
                    Direction::Up => queue.push_back(Electron{
                        x: electron.x - 1,
                        y: electron.y,
                        dir: Direction::Left, 
                        
                    }),
                    Direction::Down => queue.push_back(Electron{
                        x: electron.x + 1,
                        y: electron.y,
                        dir: Direction::Right, 
                        
                    }),
                    Direction::Left => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y - 1,
                        dir: Direction::Up, 
                        
                    }),
                    Direction::Right => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y + 1,
                        dir: Direction::Down, 
                        
                    }),
                }

            }
            '/' => {

                match electron.dir {
                    Direction::Up => queue.push_back(Electron{
                        x: electron.x + 1,
                        y: electron.y,
                        dir: Direction::Right, 
                        
                    }),
                    Direction::Down => queue.push_back(Electron{
                        x: electron.x - 1,
                        y: electron.y,
                        dir: Direction::Left, 
                        
                    }),
                    Direction::Left => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y + 1,
                        dir: Direction::Down, 
                        
                    }),
                    Direction::Right => queue.push_back(Electron{
                        x: electron.x,
                        y: electron.y - 1,
                        dir: Direction::Up, 
                        
                    }),
                }

            }
            '-' => {
                match electron.dir {
                    Direction::Left => {
                        queue.push_back(Electron{
                            x: electron.x - 1,
                            y: electron.y,
                            dir: electron.dir, 
                            
                        });
                    }
                    Direction::Right => {
                        queue.push_back(Electron{
                            x: electron.x + 1,
                            y: electron.y,
                            dir: electron.dir, 
                            
                        });
                    }
                    Direction::Up | Direction::Down => {
                        queue.push_back(Electron{
                            x: electron.x - 1,
                            y: electron.y,
                            dir: Direction::Left, 
                            
                        });
                        queue.push_back(Electron{
                            x: electron.x + 1,
                            y: electron.y,
                            dir: Direction::Right, 
                            
                        });
                    }
                }
            }
            '|' => {
                match electron.dir {
                    Direction::Up => {
                        queue.push_back(Electron{
                            x: electron.x,
                            y: electron.y - 1,
                            dir: electron.dir, 
                            
                        });
                    }
                    Direction::Down => {
                        queue.push_back(Electron{
                            x: electron.x,
                            y: electron.y + 1,
                            dir: electron.dir, 
                            
                        });
                    }
                    Direction::Left | Direction::Right => {
                        queue.push_back(Electron{
                            x: electron.x,
                            y: electron.y - 1,
                            dir: Direction::Up, 
                            
                        });
                        queue.push_back(Electron{
                            x: electron.x,
                            y: electron.y + 1,
                            dir: Direction::Down, 
                            
                        });
                    }
                }
            }
            _ => unreachable!()
        }
    }

    let unique = visited.iter()
    .map(|&e| (e.x, e.y))
    .collect::<HashSet<(i32, i32)>>()
    .iter()
    .count() as u32;

    Some(unique)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.split('\n')
    .map(|split| split.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    trace_beam(&grid, 0, 0, Direction::Right)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.split('\n')
    .map(|split| split.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let grid = Arc::new(grid);

    let g1 = grid.clone();
    let g2 = grid.clone();
    let g3 = grid.clone();
    let g4 = grid.clone();

    let threads = vec![
        thread::spawn(move || {
            let mut max = 0;
            for x in 0..g1[0].len() {
                max = max.max(trace_beam(&g1, x as i32, 0, Direction::Down).unwrap());
            }
            max
        }),
        thread::spawn(move || {
            let mut max = 0;
            for x in 0..g2[0].len() {
                max = max.max(trace_beam(&g2, x as i32, (grid.len() - 1) as i32, Direction::Up).unwrap());
            }
            max
        }),
        thread::spawn(move || {
            let mut max = 0;
            for y in 0..g3.len() {
                max = max.max(trace_beam(&g3, 0, y as i32, Direction::Right).unwrap());
            }
            max
        }),
        thread::spawn(move || {
            let mut max = 0;
            for y in 0..g4.len() {
                max = max.max(trace_beam(&g4, (g4[0].len() - 1) as i32, y as i32, Direction::Left).unwrap());
            }
            max
        }),
    ];

    let mut res = 0;

    for thread in threads {
        res = res.max(thread.join().unwrap());
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
