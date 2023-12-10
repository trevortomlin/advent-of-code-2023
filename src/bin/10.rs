#[macro_use]
extern crate lazy_static;
use std::collections::{VecDeque, HashSet, HashMap};

advent_of_code::solution!(10);

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct Neighbor {
    rx: i32,
    ry: i32,
    dir: Direction,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static NEIGHBORS: &[Neighbor] = &[
    Neighbor{rx: -1, ry: 0, dir: Direction::Left},
    Neighbor{rx: 1, ry: 0, dir: Direction::Right},
    Neighbor{rx: 0, ry: -1, dir: Direction::Up},
    Neighbor{rx: 0, ry: 1, dir: Direction::Down},
];

struct Pipe {
    north: bool,
    south: bool,
    west: bool,
    east: bool,
}

lazy_static! {
    static ref HASHMAP: HashMap<char, Pipe> = {
        let mut m = HashMap::new();
        m.insert('|', Pipe {north: true, south: true, west: false, east: false});
        m.insert('-', Pipe {north: false, south: false, west: true, east: true});
        m.insert('L', Pipe {north: true, south: false   , west: false, east: true});
        m.insert('J', Pipe {north: true, south: false, west: true, east: false});
        m.insert('7', Pipe {north: false, south: true, west: true, east: false});
        m.insert('F', Pipe {north: false, south: true, west: false, east: true});
        m
    };
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let mut start: Option<Point> = None;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start = Some(Point{
                    x: col,
                    y: row
                });
            }
        }
    }

    let start = start.expect("There should be an S in the grid!");

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut max = 0i32;

    queue.push_back((start.x as i32, start.y as i32, 0));

    while !queue.is_empty() {

        let front = queue.pop_front().unwrap();

        let (x, y, depth) = front;

        if visited.contains(&(x, y)) {
            continue;
        }
    
        visited.insert((x,y));

        max = max.max(depth);

        for neighbor in NEIGHBORS {
            if let Some(pipe) = HASHMAP.get(&grid[y as usize][x as usize]) {
                match neighbor.dir {
                    Direction::Up => {
                        if !pipe.north {
                            continue;
                        }
                    }
                    Direction::Down => {
                        if !pipe.south {
                            continue;
                        }
                    }
                    Direction::Left => {
                        if !pipe.west {
                            continue;
                        }
                    }
                    Direction::Right => {
                        if !pipe.east {
                            continue;
                        }
                    }
                }
            }

            let new_x = x + neighbor.rx;
            let new_y = y + neighbor.ry;

            if new_y < 0 || new_y >= grid.len() as i32 || new_x < 0 || new_x >= grid[0].len() as i32 {
                continue;
            }
        
            let new_char = grid[new_y as usize][new_x as usize];

            if let Some(pipe) = HASHMAP.get(&new_char) {

                let valid = match neighbor.dir {
                    Direction::Up => {
                        pipe.south
                    }
                    Direction::Down => {
                        pipe.north
                    }
                    Direction::Left => {
                        pipe.east
                    }
                    Direction::Right => {
                        pipe.west
                    }
                };

                if valid {
                    queue.push_back((new_x, new_y, depth+1));
                }

            }

        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut grid = Vec::new();

    for line in input.lines() {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let mut start: Option<Point> = None;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start = Some(Point{
                    x: col,
                    y: row
                });
            }
        }
    }

    let start = start.expect("There should be an S in the grid!");

    grid[start.y][start.x] = '|';

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut max = 0i32;

    queue.push_back((start.x as i32, start.y as i32, 0));

    while !queue.is_empty() {

        let front = queue.pop_front().unwrap();

        let (x, y, depth) = front;

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x,y));

        max = max.max(depth);

        for neighbor in NEIGHBORS {

            if let Some(pipe) = HASHMAP.get(&grid[y as usize][x as usize]) {
                match neighbor.dir {
                    Direction::Up => {
                        if !pipe.north {
                            continue;
                        }
                    }
                    Direction::Down => {
                        if !pipe.south {
                            continue;
                        }
                    }
                    Direction::Left => {
                        if !pipe.west {
                            continue;
                        }
                    }
                    Direction::Right => {
                        if !pipe.east {
                            continue;
                        }
                    }
                }
            }

            let new_x = x + neighbor.rx;
            let new_y = y + neighbor.ry;

            if new_y < 0 || new_y >= grid.len() as i32 || new_x < 0 || new_x >= grid[0].len() as i32 {
                continue;
            }
        
            let new_char = grid[new_y as usize][new_x as usize];

            if let Some(pipe) = HASHMAP.get(&new_char) {

                let valid = match neighbor.dir {
                    Direction::Up => {
                        pipe.south
                    }
                    Direction::Down => {
                        pipe.north
                    }
                    Direction::Left => {
                        pipe.east
                    }
                    Direction::Right => {
                        pipe.west
                    }
                };

                if valid {
                    queue.push_back((new_x, new_y, depth+1));
                }

            }

        }
    }

    let mut loop_grid = grid.clone();

    for row in 0..loop_grid.len() {
        for col in 0..loop_grid[0].len() {
            if !visited.contains(&(col as i32, row as i32)) {
                loop_grid[row][col] = '.';
            }
        }
    }

    loop_grid[start.y][start.x] = '|';

    let mut ctr = 0;

    for row in 0..loop_grid.len() {
        for col in 0..loop_grid[0].len() {
            let mut fl_char = ' ';
            let mut tmp = 0;

            if visited.contains(&(col as i32, row as i32)) {
                continue;
            }

            for l in 0..col {
            
                if visited.contains(&(l as i32, row as i32)) {
                    if loop_grid[row][l] == '|' {
                        tmp += 1;
                    }
                    if loop_grid[row][l] == '-' {
                        continue;
                    }
                    else if loop_grid[row][l] == 'F' {
                        fl_char = 'F';
                    }
                    else if loop_grid[row][l] == 'L' {
                        fl_char = 'L';
                    }
                    else if loop_grid[row][l] == 'J' {
                        if fl_char == 'F' {
                            tmp += 1;
                        }
                        fl_char = ' ';
                    }
                    else if loop_grid[row][l] == '7' {
                        if fl_char == 'L' {
                            tmp += 1;
                        }
                        fl_char = ' ';
                    }

                }

            }

            if tmp % 2 == 1 {
                ctr += 1;
            }

        }

    }

    Some(ctr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two3() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(10));
    }

}