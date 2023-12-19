advent_of_code::solution!(18);

use itertools::{self, Itertools};

pub fn part_one(input: &str) -> Option<i32> {
    let mut items: Vec<(&str, i32, &str)> = Vec::new();

    for line in input.split('\n') {
        let split: Vec<&str> = line.split(' ').collect();
        let dir = split[0];
        let cnt = split[1].parse::<i32>().unwrap();
        let color = split[2];
        items.push((dir, cnt, color));
    }

    let mut points: Vec<(i32, i32)> = vec![(0, 0)];
    let mut cur = (0, 0); 
    let mut perim = 0;

    for (dir, cnt, _) in items {
        perim += cnt;
        match dir {
            "U" => {
                cur = (cur.0, cur.1 - 1 * cnt);
                points.push(cur);
            },
            "D" => {
                cur = (cur.0, cur.1 + 1 * cnt);
                points.push(cur);
            },
            "L" => {
                cur = (cur.0 - 1 * cnt, cur.1);
                points.push(cur);
            },
            "R" => {
                cur = (cur.0 + 1 * cnt, cur.1);
                points.push(cur);   
            },
            _ => unreachable!()
        }
    }

    let mut area: i32 = points.iter().tuple_windows()
    .map(|(p1, p2)| {
        p1.0 * p2.1 - p1.1 * p2.0
    }).sum::<i32>();

    let p1 = points.last().unwrap();
    let p2 = points.first().unwrap();

    area += p1.0 * p2.1 - p1.1 * p2.0;

    area /= 2;

    Some(area + perim / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut items: Vec<(&str, i64)> = Vec::new();

    for line in input.split('\n') {
        let split: Vec<&str> = line.split(' ').collect();
        let color = split[2];

        let cnt = i64::from_str_radix(&color[2..=6], 16).unwrap();
        let dir = match color.as_bytes()[7] {
            b'0' => "R",
            b'1' => "D",
            b'2' => "L",
            b'3' => "U",
            _ => unreachable!(),
        };

        items.push((dir, cnt));
    }

    let mut points: Vec<(i64, i64)> = vec![(0, 0)];
    let mut cur = (0, 0); 
    let mut perim = 0;

    for (dir, cnt) in items {
        perim += cnt;
        match dir {
            "U" => {
                cur = (cur.0, cur.1 - 1 * cnt);
                points.push(cur);
            },
            "D" => {
                cur = (cur.0, cur.1 + 1 * cnt);
                points.push(cur);
            },
            "L" => {
                cur = (cur.0 - 1 * cnt, cur.1);
                points.push(cur);
            },
            "R" => {
                cur = (cur.0 + 1 * cnt, cur.1);
                points.push(cur);   
            },
            _ => unreachable!()
        }
    }

    let mut area: i64 = points.iter().tuple_windows()
    .map(|(p1, p2)| {
        (p1.0 * p2.1 - p1.1 * p2.0) as i64
    }).sum::<i64>();

    let p1 = points.last().unwrap();
    let p2 = points.first().unwrap();

    area += (p1.0 * p2.1 - p1.1 * p2.0) as i64;

    area /= 2;

    Some(area + perim as i64 / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
