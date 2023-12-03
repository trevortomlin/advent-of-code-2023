advent_of_code::solution!(2);

use regex::Regex;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>
}

#[derive(Debug, Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_one(input: &str) -> Option<u32> {

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let re = Regex::new(
        r"\d+"
    ).unwrap();

    let re2 = Regex::new(
        r"\d* (blue|red|green)"
    ).unwrap();

    let res: u32 = input.trim().split('\n').map(|s| {

        let sets = s.split(';').map(|set| {
            let sets_re = re2.find_iter(set).map(|m| {
                let m_str = m.as_str();
                m_str.split_at(m_str.find(' ').unwrap())
            }).collect::<Vec<(&str, &str)>>();

            let mut out = Set::default();

            for (amt, color) in sets_re {
                match color.trim() {
                    "red" => out.red += amt.parse::<u32>().unwrap(),
                    "blue" => out.blue += amt.parse::<u32>().unwrap(),
                    "green" => out.green += amt.parse::<u32>().unwrap(),
                    _ => unreachable!()
                };
            }
            out
        }).collect::<Vec<Set>>();

        Game {
            id: re.find(s).unwrap().as_str().parse::<u32>().unwrap(),
            sets,
        }

    })
    .filter(|game| {
        game.sets.iter().all(|set| {
            set.red <= MAX_RED &&
            set.blue <= MAX_BLUE &&
            set.green <= MAX_GREEN
        })
    })
    .map(|game| {
        game.id
    })
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(
        r"\d+"
    ).unwrap();

    let re2 = Regex::new(
        r"\d* (blue|red|green)"
    ).unwrap();

    let res: u32 = input.trim().split('\n').map(|s| {

        let sets = s.split(';').map(|set| {
            let sets_re = re2.find_iter(set).map(|m| {
                let m_str = m.as_str();
                m_str.split_at(m_str.find(' ').unwrap())
            }).collect::<Vec<(&str, &str)>>();

            let mut out = Set::default();

            for (amt, color) in sets_re {
                match color.trim() {
                    "red" => out.red += amt.parse::<u32>().unwrap(),
                    "blue" => out.blue += amt.parse::<u32>().unwrap(),
                    "green" => out.green += amt.parse::<u32>().unwrap(),
                    _ => unreachable!()
                };
            }
            out
        }).collect::<Vec<Set>>();

        Game {
            id: re.find(s).unwrap().as_str().parse::<u32>().unwrap(),
            sets,
        }

    })
    .map(|game| {

        let max_red = game.sets.iter().fold(0, |max, val| if val.red > max{ val.red } else{ max });
        let max_green = game.sets.iter().fold(0, |max, val| if val.green > max{ val.green } else{ max });
        let max_blue = game.sets.iter().fold(0, |max, val| if val.blue > max{ val.blue } else{ max });

        Set {
            red: max_red,
            green: max_green,
            blue: max_blue
        }
    })
    .map(|set| {
        set.red * set.blue * set.green
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
