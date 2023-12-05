advent_of_code::solution!(5);

use rayon::prelude::*;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let numbers_re = Regex::new(
        r"\d+"
    ).unwrap();

    let seeds_re = Regex::new(
        r"(:)(.|\n)*?(\n\n)"
    ).unwrap();

    let mut rows =seeds_re.find_iter(input).map(|m| {
        numbers_re.find_iter(m.as_str()).map(|num| {
            num.as_str().parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let seeds = rows.remove(0);

    let mut min = u32::MAX;

    for seed in seeds {
        let mut val = seed;
        for row in &rows {

            for chunk in row.chunks_exact(3) {
                let dest = chunk[0];
                let source = chunk[1];
                let len = chunk[2];

                if val >= source && val <= source + len {
                    let diff = val - source;
                    let new_val = dest + diff;
                    val = new_val;
                    break;
                }
            }

        }

        min = min.min(val);

    }

    Some(min)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers_re = Regex::new(
        r"\d+"
    ).unwrap();

    let seeds_re = Regex::new(
        r"(:)(.|\n)*?(\n\n)"
    ).unwrap();

    let mut rows =seeds_re.find_iter(input).map(|m| {
        numbers_re.find_iter(m.as_str()).map(|num| {
            num.as_str().parse::<u64>().unwrap()
        }).collect::<Vec<u64>>()
    }).collect::<Vec<Vec<u64>>>();

    let seed_ranges = rows.remove(0);

    let seeds: Vec<_> = seed_ranges.chunks(2).map(|range| {
        (range[0]..range[0]+range[1]).collect::<Vec<u64>>()
    })
    .flatten()
    .collect();

    let min = seeds.par_iter().cloned().map(|seed| {
        let mut val = seed;
        for row in &rows {

            for chunk in row.chunks_exact(3) {
                let dest = chunk[0];
                let source = chunk[1];
                let len = chunk[2];

                if val >= source && val < source + len {
                    let diff = val - source;
                    let new_val = dest + diff;
                    val = new_val;
                    break;
                }
            }

        }

        return val;

    }).min().unwrap();

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
