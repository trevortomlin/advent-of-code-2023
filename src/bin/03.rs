advent_of_code::solution!(3);

use std::collections::HashMap;

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"\d+"
    ).unwrap();
    
    let res: Vec<Vec<char>> = input.trim()
    .split_inclusive("\n")
    .map(|s| {
        s.chars().collect()
    }).collect::<Vec<Vec<char>>>();

    let width = res[0].len();
    let height = res.len();

    let i_to_xy = |i: usize, width: usize| -> (usize, usize) {
        let x = i % width;
        let y = i / width;
        (x, y)
    };

    let nums = re.find_iter(&input).map(|m| {
        let s = m.as_str();
        let start = m.start();
        let end = m.end();
        (s, start, end)
    }).collect::<Vec<(&str, usize, usize)>>();

    let res: u32 = nums.iter()
    .filter(|(s, start, end)| {
        let start_xy = i_to_xy(*start, width);
        let end_xy = i_to_xy(*end, width);

        let mut neighbors = Vec::new();

        if start_xy.0 > 0 {
            neighbors.push((start_xy.0-1, start_xy.1));
        }
        if end_xy.0 < width - 1 && end_xy.1 < height {
            neighbors.push((end_xy.0, end_xy.1));
        }

        //top and bot row
        for x in start_xy.0 as i32 - 1..=end_xy.0 as i32 {
            
            if x >= 0 && x <= (width as i32 - 1) {

                if start_xy.1 > 0 {
                    neighbors.push((x as usize, start_xy.1 - 1)); 
                }
                if start_xy.1 < height - 1 {
                    neighbors.push((x as usize, start_xy.1 + 1));
                }

            } 
        }

        for (x, y) in neighbors {

            let elem = res[y][x];

            if elem.is_ascii_punctuation() && elem != '.' {
                return true;
            } 
        }

        false
    })
    .map(|(s, _, _)| {
        s.parse::<u32>().unwrap()
    })
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut hashmap = HashMap::<(usize, usize), Vec<u32>>::new();

    let re = Regex::new(
        r"\d+"
    ).unwrap();
    
    let res: Vec<Vec<char>> = input.trim()
    .split_inclusive("\n")
    .map(|s| {
        s.chars().collect()
    }).collect::<Vec<Vec<char>>>();

    let width = res[0].len();
    let height = res.len();

    let i_to_xy = |i: usize, width: usize| -> (usize, usize) {
        let x = i % width;
        let y = i / width;
        (x, y)
    };

    let nums = re.find_iter(&input).map(|m| {
        let s = m.as_str();
        let start = m.start();
        let end = m.end();
        (s, start, end)
    }).collect::<Vec<(&str, usize, usize)>>();

    for (s, start, end) in nums.iter() {
        let start_xy = i_to_xy(*start, width);
        let end_xy = i_to_xy(*end, width);

        let mut neighbors = Vec::new();

        if start_xy.0 > 0 {
            neighbors.push((start_xy.0-1, start_xy.1));
        }
        if end_xy.0 < width - 1 && end_xy.1 < height {
            neighbors.push((end_xy.0, end_xy.1));
        }

        //top and bot row
        for x in start_xy.0 as i32 - 1..=end_xy.0 as i32 {
            
            if x >= 0 && x <= (width as i32 - 1) {

                if start_xy.1 > 0 {
                    neighbors.push((x as usize, start_xy.1 - 1)); 
                }
                if start_xy.1 < height - 1 {
                    neighbors.push((x as usize, start_xy.1 + 1));
                }

            } 
        }

        for (x, y) in neighbors {

            let elem = res[y][x];

            if elem == '*' {

                if !hashmap.contains_key(&(x,y)) {
                    hashmap.insert((x,y), Vec::new());
                }

                hashmap.get_mut(&(x,y)).unwrap().push(s.parse::<u32>().unwrap());
            } 
        }
    }

    let res: u32 = hashmap.iter()
    .filter(|(_, v)| {
        v.len() == 2
    })
    .map(|(_, v)| {
        v[0] * v[1]
    })
    .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
