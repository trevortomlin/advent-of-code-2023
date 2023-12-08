advent_of_code::solution!(4);

use std::collections::HashSet;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"\d+"
    ).unwrap();

    let res: u32 = input.trim().split('\n').map(|s| {
        let (winning, have) = s.split_once('|').unwrap();

        let winning = winning.split_once(':').unwrap().1;

        let nums_winning: Vec<u32> = re.find_iter(winning).map(|m| {
            m.as_str().parse::<u32>().unwrap()
        }).collect();

        let nums_have: Vec<u32> = re.find_iter(have).map(|m| {
            m.as_str().parse::<u32>().unwrap()
        }).collect();

        let mut hash_set_winning = HashSet::<u32>::new();
        let mut hash_set_have = HashSet::<u32>::new();

        for num in nums_winning {
            hash_set_winning.insert(num);
        }

        for num in nums_have {
            hash_set_have.insert(num);
        }

        let count = hash_set_winning.intersection(&hash_set_have).count();

        let mut out = 0;
        for _ in 0..count {
            if out == 0 {
                out = 1;
            }
            else {
                out *= 2;
            }
        }

        out

    }).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"\d+"
    ).unwrap();

    let mut cards: Vec<u32> = Vec::new();

    for (i, s) in input.trim().split('\n').enumerate() {
        let (winning, have) = s.split_once('|').unwrap();

        let winning = winning.split_once(':').unwrap().1;

        let nums_winning: Vec<u32> = re.find_iter(winning).map(|m| {
            m.as_str().parse::<u32>().unwrap()
        }).collect();

        let nums_have: Vec<u32> = re.find_iter(have).map(|m| {
            m.as_str().parse::<u32>().unwrap()
        }).collect();

        let mut hash_set_winning = HashSet::<u32>::new();
        let mut hash_set_have = HashSet::<u32>::new();

        for num in nums_winning {
            hash_set_winning.insert(num);
        }

        for num in nums_have {
            hash_set_have.insert(num);
        }

        let card_count = match cards.get_mut(i) {
            Some(val) => {
                *val += 1;
                *val
            }
            None => {
                cards.push(1);
                1
            }
        };  
        let matches_count = hash_set_winning.intersection(&hash_set_have).count() as u32;

        for x in 1..=matches_count as usize{
            match cards.get_mut(i + x) {
                Some(val) => *val+=card_count,
                None => cards.push(card_count) 
            }
        }

    }

    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}