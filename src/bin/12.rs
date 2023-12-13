advent_of_code::solution!(12);

use std::collections::{VecDeque, HashMap};

fn count(line: &str, groups: &mut VecDeque<u64>, memo: &mut HashMap<(String, VecDeque<u64>), u64> ) -> u64 {

    if let Some(cached_value) = memo.get(&(line.to_string(), groups.clone())) {
        return *cached_value;
    }

    if line.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        else {
            return 0;
        }
    }

    if groups.is_empty()  {
        if line.contains("#") {
            return 0;
        }
        else {
            return 1;
        }
    }

    let mut res = 0;

    if line.as_bytes()[0] == b'.' || line.as_bytes()[0] == b'?' {
        res += count(&line[1..], groups, memo);
    }

    if line.as_bytes()[0] == b'#' || line.as_bytes()[0] == b'?' {
        let first = *groups.front().unwrap() as usize;

        if first < line.len() 
        && !line[..first].contains('.') 
        && (first == line.len() || line.as_bytes()[first] != b'#') {
            let mut tmp = groups.clone();
            tmp.pop_front();
            res += count(&line[first+1..], &mut tmp, memo);
        }
    }


    memo.insert((line.to_string(), groups.clone()), res);


    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let splits: Vec<(String, VecDeque<u64>)> = input.split('\n')
    .map(|s| {
        let split: Vec<&str> = s.split(' ').collect();

        let mut t = split[0].to_string();
        t.push_str(".");

        (t, split[1].split(',').map(|s| s.parse().unwrap()).collect())
    }).collect();

    let mut sum = 0;

    for (n1, n2) in splits {
        let mut n2 = n2.clone();
        let mut memo: HashMap<(String, VecDeque<u64>), u64> = HashMap::new();
        let res = count(&n1, &mut n2, &mut memo);
        sum += res;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let splits: Vec<(String, VecDeque<u64>)> = input.split('\n')
    .map(|s| {
        let split: Vec<&str> = s.split(' ').collect();

        let mut t = split[0].to_string();
        t.push_str("?");
        t = t.repeat(5);
        t = t[..t.len()-1].to_string();
        t.push_str(".");

        let p: Vec<u64> = split[1].split(',').map(|s| s.parse().unwrap()).collect();
        let p = std::iter::repeat(p.iter()).take(5).flatten().map(|u| *u).collect();

        (t, p)
    }).collect();

    let mut sum = 0;

    for (n1, n2) in splits {
        let mut n2 = n2.clone();
        let mut memo: HashMap<(String, VecDeque<u64>), u64> = HashMap::new();
        let res = count(&n1, &mut n2, &mut memo);
        sum += res;
    }

    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
