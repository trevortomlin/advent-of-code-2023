advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let blocks = input.split("\n\n");

    let blocks: Vec<Vec<Vec<char>>> = blocks.map(|b| b.split("\n")
    .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()).collect();

    let mut rows = 0;
    let mut cols = 0;

    for block in &blocks {
        for i in 1..block.len() {

            let before = &block[..i];
            let after = &block[i..];

            let is_mirror: bool = before.iter().rev()
            .zip(after.iter())
            .all(|(&ref b, &ref a)| { b == a });

            if is_mirror {
                rows += i as u32;
                break;
            }

        }

        let block_transpose = block.clone();

        let r = block_transpose.len();
        let c = block_transpose[0].len();

        let transposed: Vec<Vec<_>> = (0..c).map(|col| {
            (0..r)
                .map(|row| block_transpose[row][col])
                .collect()
        }).collect();

        for i in 1..transposed.len() {

            let before = &transposed[..i];
            let after = &transposed[i..];

            let is_mirror: bool = before.iter().rev()
            .zip(after.iter())
            .all(|(&ref b, &ref a)| { b == a });

            if is_mirror {
                cols += i as u32;
                break;
            }

        }

        
    }

    Some(cols + 100 * rows)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blocks = input.split("\n\n");

    let blocks: Vec<Vec<Vec<char>>> = blocks.map(|b| b.split("\n")
    .map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()).collect();

    let mut rows = 0;
    let mut cols = 0;

    for block in &blocks {
        for i in 1..block.len() {

            let before = &block[..i];
            let after = &block[i..];

            let is_mirror: bool = before.iter().rev()
            .zip(after.iter())
            .map(|(&ref b, &ref a)| {
                b.iter().zip(a).filter(|(&b, &a)| { a != b}).count() as u64
            })
            .sum::<u64>() == 1;

            if is_mirror {
                rows += i as u32;
                break;
            }

        }

        let block_transpose = block.clone();

        let r = block_transpose.len();
        let c = block_transpose[0].len();

        let transposed: Vec<Vec<_>> = (0..c).map(|col| {
            (0..r)
                .map(|row| block_transpose[row][col])
                .collect()
        }).collect();

        for i in 1..transposed.len() {

            let before = &transposed[..i];
            let after = &transposed[i..];

            let is_mirror: bool = before.iter().rev()
            .zip(after.iter())
            .map(|(&ref b, &ref a)| {
                b.iter().zip(a).filter(|(&b, &a)| { a != b}).count() as u64
            })
            .sum::<u64>() == 1;

            if is_mirror {
                cols += i as u32;
                break;
            }

        }

        
    }

    Some(cols + 100 * rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
