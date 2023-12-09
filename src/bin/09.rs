advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut result: Vec<i32> = Vec::new();

    for line in input.lines() {

        let nums = line.split(" ")
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

        let mut v: Vec<Vec<i32>> = Vec::new();

        let mut cur = nums.clone();

        v.push(cur.clone());

        while !cur.iter().all(|c| *c == 0) {
            cur = cur.windows(2).map(|w| w[1] - w[0]).collect();
            v.push(cur.clone());
        }   

        let sum = v.iter().rev().map(|row| *row.last().unwrap()).fold(0, |acc, a| acc+a);

        result.push(sum);

    }

    let result = result.iter().sum();

    Some(result)

}

pub fn part_two(input: &str) -> Option<i32> {
    let mut result: Vec<i32> = Vec::new();

    for line in input.lines() {

        let nums = line.split(" ")
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

        let mut v: Vec<Vec<i32>> = Vec::new();

        let mut cur = nums.clone();

        v.push(cur.clone());

        while !cur.iter().all(|c| *c == 0) {
            cur = cur.windows(2).map(|w| w[1] - w[0]).collect();
            v.push(cur.clone());
        }   
        
        let sum = v.iter().rev().map(|row| *row.first().unwrap()).fold(0, |acc, a| a-acc);

        result.push(sum);

    }

    let result = result.iter().sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
