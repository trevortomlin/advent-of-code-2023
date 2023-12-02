advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    
    let x: u32 = input.trim().split("\n").map(|s| {
        s.chars().filter(|&c| {
            match c {
                '0'..='9' => true,
                _ => false,
            }
        }).collect::<Vec<char>>()
    })
    .map(|v| {
        let mut c1 = v.first().unwrap().to_string();
        let c2 = v.last().unwrap();
        c1.push(*c2);
        c1.parse::<u32>().unwrap()
    })
    .sum();

    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {

    let map_to_str = |s: &str| -> Option<&str> {
        match s {
            "one" => Some("1"),
            "two" => Some("2"),
            "three" => Some("3"),
            "four" => Some("4"),
            "five" => Some("5"),
            "six" => Some("6"),
            "seven" => Some("7"),
            "eight" => Some("8"),
            "nine" => Some("9"),
            _ => None,
        }
    };

    let re = Regex::new(
        r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine"
    ).unwrap();

    let x: u32 = input.trim().split("\n").map(|s| {
        let first = re.find(s).unwrap();

        let mut last = first.as_str();
        let mut idx = s.len() - 1;

        loop {
            if let Some(re_match) = re.find(&s[idx..]) {
               last = re_match.as_str();
               break;
            }
            else {
                idx-=1;
            }

            if idx == 0 {
                break;
            }
        }

        let first = first.as_str();

        let mut out = String::new();

        if let Some(str) = map_to_str(first) {
            out.push_str(str);
        }
        else {
            out.push_str(first);
        }

        if let Some(str) = map_to_str(last) {
            out.push_str(str);
        }
        else {
            out.push_str(last);
        }

        out.parse::<u32>().unwrap()

    }).sum();

    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
