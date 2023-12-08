advent_of_code::solution!(8);

use std::collections::HashMap;
use regex::Regex;		

pub fn part_one(input: &str) -> Option<u32> {
    let words_re = Regex::new(
        r"[A-Z]+"
    ).unwrap();

    let mut splits: Vec<_> = words_re.find_iter(input)
    .map(|m| m.as_str())
    .collect();
    
    let words = splits.split_off(1);
    let dirs = splits[0];

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for c in words.chunks_exact(3) {
        let start = c[0];
        let left = c[1];
        let right = c[2];
        map.insert(start, (left, right));
    }

    let mut index = 0;
    let mut cur = "AAA";

    while cur != "ZZZ" {
        if let Some(v) = map.get(cur) {
            
            match dirs.chars().nth(index % dirs.len()).unwrap() {
                'L' => cur = v.0,
                'R' => cur = v.1,
                _ => unreachable!()
            };

            index += 1;

        }
    }

    Some(index as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let words_re = Regex::new(
        r"[0-9A-Z]+"
    ).unwrap();

    let mut splits: Vec<_> = words_re.find_iter(input)
    .map(|m| m.as_str())
    .collect();
    
    let words = splits.split_off(1);
    let dirs = splits[0];

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for c in words.chunks_exact(3) {
        let start = c[0];
        let left = c[1];
        let right = c[2];
        map.insert(start, (left, right));
    }
    
    let mut m = Vec::new();

    let mut index = 0;

    let starts: Vec<_> = map.keys().filter(|&&n| n.ends_with('A')).collect();

    for &node in starts {
        let mut cur = node;
        index = 0;

        while !cur.ends_with('Z'){
            if let Some(v) = map.get(cur) {
                
                match dirs.chars().nth(index % dirs.len()).unwrap() {
                    'L' => cur = v.0,
                    'R' => cur = v.1,
                    _ => unreachable!()
                };

                index+=1;
        

            }

        }

        m.push(index);

    }

    let res = m.iter().map(|c| *c as u64).reduce(|a, b| {
        num::integer::lcm(a, b)
    }).unwrap();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
