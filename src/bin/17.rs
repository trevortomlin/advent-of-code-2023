advent_of_code::solution!(17);

use std::{hash::Hasher, collections::{BinaryHeap, HashSet}, cmp::Ordering};
use core::hash::Hash;

#[derive(Eq, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    dir: (i32, i32),
    steps: i32,
    heat: i32,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y 
        && self.steps == other.steps
        && self.dir == other.dir
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.heat.cmp(&other.heat))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heat.cmp(&other.heat)
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.steps.hash(state);
        self.dir.hash(state);
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = input.split('\n')
    .map(|split| split.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
    .collect::<Vec<Vec<u32>>>();

    let mut visited: HashSet<Tile> = HashSet::new();
    let mut queue: BinaryHeap<Tile> = BinaryHeap::new();

    let max_steps = 3;

    let in_bounds = |x: i32, y: i32| -> bool {
        return x >= 0 && x < (grid[0].len()) as i32 
            && y >= 0 && y < (grid.len()) as i32;
    };

    let n1 = Tile {
        x: 1,
        y: 0,
        dir: (1, 0),
        steps: 0,
        heat: -(grid[0][1] as i32),
    };

    let n2 = Tile {
        x: 0,
        y: 1,
        dir: (0, 1),
        steps: 0,
        heat: -(grid[1][0] as i32),
    };

    queue.push(n1);
    queue.push(n2);

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        if cur.x == (grid[0].len() - 1) as i32 && cur.y == (grid.len() - 1) as i32 {
            return Some(-1 * cur.heat);
        }
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        if cur.steps < (max_steps - 1) && in_bounds(cur.x + cur.dir.0, cur.y + cur.dir.1) {
            let new = Tile {
                x: cur.x + cur.dir.0,
                y: cur.y + cur.dir.1,
                dir: cur.dir,
                steps: cur.steps + 1,
                heat: cur.heat - grid[(cur.y + cur.dir.1) as usize][(cur.x + cur.dir.0) as usize] as i32,
            };
            queue.push(new);
        }

        if in_bounds(cur.x + cur.dir.1, cur.y + -cur.dir.0) {
            let new = Tile {
                x: cur.x + cur.dir.1,
                y: cur.y + -cur.dir.0,
                dir: (cur.dir.1, -cur.dir.0),
                steps: 0,
                heat: cur.heat - grid[(cur.y - cur.dir.0) as usize][(cur.x + cur.dir.1) as usize] as i32,
            };
            queue.push(new);
        }

        if in_bounds(cur.x + -cur.dir.1, cur.y + cur.dir.0) {
            let new = Tile {
                x: cur.x + -cur.dir.1,
                y: cur.y + cur.dir.0,
                dir: (-cur.dir.1, cur.dir.0),
                steps: 0,
                heat: cur.heat - grid[(cur.y + cur.dir.0) as usize][(cur.x + -cur.dir.1) as usize] as i32,
            };
            queue.push(new);
        }

    }

    None

}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = input.split('\n')
    .map(|split| split.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
    .collect::<Vec<Vec<u32>>>();

    let mut visited: HashSet<Tile> = HashSet::new();
    let mut queue: BinaryHeap<Tile> = BinaryHeap::new();

    let min_steps = 3;
    let max_steps = 10;

    let in_bounds = |x: i32, y: i32| -> bool {
        return x >= 0 && x < (grid[0].len()) as i32 
            && y >= 0 && y < (grid.len()) as i32;
    };

    let n1 = Tile {
        x: 1,
        y: 0,
        dir: (1, 0),
        steps: 0,
        heat: -(grid[0][1] as i32),
    };

    let n2 = Tile {
        x: 0,
        y: 1,
        dir: (0, 1),
        steps: 0,
        heat: -(grid[1][0] as i32),
    };

    queue.push(n1);
    queue.push(n2);

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();

        if cur.x == (grid[0].len() - 1) as i32 && cur.y == (grid.len() - 1) as i32 {
            return Some(-1 * cur.heat);
        }
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        if cur.steps < (max_steps - 1) && in_bounds(cur.x + cur.dir.0, cur.y + cur.dir.1) {
            let new = Tile {
                x: cur.x + cur.dir.0,
                y: cur.y + cur.dir.1,
                dir: cur.dir,
                steps: cur.steps + 1,
                heat: cur.heat - grid[(cur.y + cur.dir.1) as usize][(cur.x + cur.dir.0) as usize] as i32,
            };
            queue.push(new);
        }

        if cur.steps >= min_steps {
            if in_bounds(cur.x + cur.dir.1, cur.y + -cur.dir.0) {
                let new = Tile {
                    x: cur.x + cur.dir.1,
                    y: cur.y + -cur.dir.0,
                    dir: (cur.dir.1, -cur.dir.0),
                    steps: 0,
                    heat: cur.heat - grid[(cur.y - cur.dir.0) as usize][(cur.x + cur.dir.1) as usize] as i32,
                };
                queue.push(new);
            }

            if in_bounds(cur.x + -cur.dir.1, cur.y + cur.dir.0) {
                let new = Tile {
                    x: cur.x + -cur.dir.1,
                    y: cur.y + cur.dir.0,
                    dir: (-cur.dir.1, cur.dir.0),
                    steps: 0,
                    heat: cur.heat - grid[(cur.y + cur.dir.0) as usize][(cur.x + -cur.dir.1) as usize] as i32,
                };
                queue.push(new);
            }
        }

    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
