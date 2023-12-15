advent_of_code::solution!(15);

fn hash(s: &str) -> u32 {
    let mut res = 0;

    for &char in s.as_bytes() {
        if char == b'\n' {
            continue;
        }
        res += char as u32;
        res *= 17;
        res %= 256;
    }

    res
}

pub fn part_one(input: &str) -> Option<u32> {

    let res: u32 = input.split(',')
    .map(|s| hash(s))
    .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for op in input.split(',') {
        // Debug output same as example
        // println!("After {}", op);

        match op.contains('=') {
            true => {
                let (lens, cnt) = op.split_once('=').unwrap();
                let cnt = cnt.parse::<u32>().unwrap();

                let i = hash(lens) as usize;

                if let Some(item) = boxes[i].iter_mut().find(|o| o.0 == lens) {
                    *item = (lens, cnt);
                }
                else {
                    boxes[i].push((lens, cnt));
                }

            },
            false => {
                let lens = &op[..op.len()-1];
                let i = hash(lens) as usize;
                boxes[i].retain(|&l| l.0 != lens);
            },
        }

        // Debug output same as example
        // for (i, j) in boxes.iter().enumerate() {
        //     if j.is_empty() {
        //         continue;
        //     }
        //     print!("Box {}: ", i);
        //     for s in j {
        //         print!("[{} {}] ", s.0, s.1);
        //     }

        //     println!();

        // }

        // println!();

    }

    let res: u32 = boxes.iter().enumerate().flat_map(|(i, j)| {
        j.iter()
        .enumerate()
        .map(|(ji, s)| {
            (i as u32 + 1) * (ji  as u32 + 1) * s.1
        })
        .collect::<Vec<u32>>()
    }).sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
