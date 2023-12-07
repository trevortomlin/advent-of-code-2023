advent_of_code::solution!(7);

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq)]
enum CmpType {
    One,
    Two,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand<'a> {
    card: &'a str,
    hand_type: Type,
    bid: u32,
    cmp_type: CmpType,
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {

        let cmp_func = match self.cmp_type {
            CmpType::One => char_to_strength,
            CmpType::Two => char_to_strength2,
        };

        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {

                let mut out = Ordering::Less;

                for (c1, c2) in self.card.chars().zip(other.card.chars()) {
                    if c1 == c2 {
                        continue;
                    }

                    if cmp_func(c1) > cmp_func(c2) {
                        out = Ordering::Greater;
                        break;
                    }

                    out = Ordering::Less;
                    break;
                    
                }

                out
            },
            Ordering::Greater => Ordering::Greater,
        }
    }
}

fn char_to_strength(char: char) -> u32 {
    match char {
        '2'..='9' => char.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!()
    }
}

fn char_to_strength2(char: char) -> u32 {
    match char {
        'J' => 0,
        '2'..='9' => char.to_digit(10).unwrap(),
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let splits = input.split('\n').map(|line| {
        let split: Vec<_> = line.split(' ').collect();
        (split[0], split[1].parse::<u32>().unwrap())
    }).collect::<Vec<_>>();

    let mut cards: Vec<Hand> = Vec::new();

    for (card, bid) in splits {
        let mut letter_counts: HashMap<char,i32> = HashMap::new();

        for c in card.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        let card_type = match letter_counts.len() {
            1 => Type::FiveOfAKind,
            2 => {

                match letter_counts.into_values().any(|x| x == 4) {
                    true => Type::FourOfAKind,
                    false => Type::FullHouse
                }
            }
            3 => {
                match letter_counts.into_values().any(|x| x == 3) {
                    true => Type::ThreeOfAKind,
                    false => Type::TwoPair
                }
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unreachable!()
        };

        let hand = Hand {
            card: card,
            hand_type: card_type,
            bid: bid,
            cmp_type: CmpType::One,
        };

        cards.push(hand);
    }

    cards.sort();

    let result = cards.iter().enumerate().map(|(i, c)| {
        (i as u32 + 1) * c.bid
    }).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let splits = input.split('\n').map(|line| {
        let split: Vec<_> = line.split(' ').collect();
        (split[0], split[1].parse::<u32>().unwrap())
    }).collect::<Vec<_>>();

    let mut cards: Vec<Hand> = Vec::new();

    for (card, bid) in splits {
        let mut letter_counts: HashMap<char,i32> = HashMap::new();

        for c in card.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        if letter_counts.len() == 1 && letter_counts.get(&'J').is_some() {
            let hand = Hand {
                card: "JJJJJ",
                hand_type: Type::FiveOfAKind,
                bid: bid,
                cmp_type: CmpType::Two
            };
            cards.push(hand);
            continue;
        }

        let max_key = *letter_counts.iter()
        .filter(|(&k, _)| {
            k != 'J'
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

        let joker_count = match letter_counts.get(&'J') {
            Some(v) => *v,
            None => 0,
        };

        letter_counts.remove(&'J');

        *letter_counts.entry(max_key).or_insert(0) += joker_count as i32;

        let card_type = match letter_counts.len() {
            1 => Type::FiveOfAKind,
            2 => {

                match letter_counts.into_values().any(|x| x == 4) {
                    true => Type::FourOfAKind,
                    false => Type::FullHouse
                }
            }
            3 => {
                match letter_counts.into_values().any(|x| x == 3) {
                    true => Type::ThreeOfAKind,
                    false => Type::TwoPair
                }
            }
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => unreachable!()
        };

        let hand = Hand {
            card: card,
            hand_type: card_type,
            bid: bid,
            cmp_type: CmpType::Two
        };

        cards.push(hand);
    }

    cards.sort();

    let result = cards.iter().enumerate().map(|(i, c)| {
        (i as u32 + 1) * c.bid
    }).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_all_cards() {
        let input = "AAAAA 1\nAA8AA 1\n23332 1\nTTT98 1\n23432 1\nA23A4 1\n23456 1";
        part_one(input);
    }

    #[test]
    fn test_four_of_a_kind() {
        let input = "AJTTT 1";
        part_two(input);
    }

}
