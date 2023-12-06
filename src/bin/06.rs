advent_of_code::solution!(6);

use::regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {

    let numbers_re = Regex::new(
        r"\d+"
    ).unwrap();

    let mut time = numbers_re.find_iter(input).map(|m| {m.as_str()}).collect::<Vec<_>>();
    let distance = time.split_off(time.len() / 2);

    let res = time.iter().zip(distance).map(|(&t, d)| {
        let t = t.parse::<u32>().unwrap();
        let d = d.parse::<u32>().unwrap();

        (0..=t).collect::<Vec<_>>().iter().map(|x| {
            let diff = t - x;
            x * diff
        }).filter(|x| {
            *x > d
        }).count() as u32

    }).product();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.split("\n").collect::<Vec<_>>();
    let time = lines[0];
    let distance = lines[1];

    let time = time.chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap();
    let distance = distance.chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap();

    Some(
        (0..=time).collect::<Vec<_>>().iter().map(|x| {
            let diff = time - x;
            x * diff
        }).filter(|x| {
            *x > distance
        }).count() as u64
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
