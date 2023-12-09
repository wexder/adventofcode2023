use core::fmt;
use std::{fs, path::Path};

pub fn run<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return cards(data).iter().sum();
}

fn cards(input: String) -> Vec<i32> {
    input
        .split("\n")
        .filter(|&line| line.ne(""))
        .map(parse_card)
        .map(|card| Card::get_points(&card))
        .collect()
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return pile(data);
}

fn pile(input: String) -> i32 {
    let mut cards: Vec<Card> = input
        .split("\n")
        .filter(|&line| line.ne(""))
        .map(parse_card)
        .collect();
    cards.sort_by(|a, b| a.id.cmp(&b.id));

    let mut card_count = 0;
    cards
        .iter()
        .for_each(|c| recursive_winning(c, &cards, &mut card_count));

    card_count
}

// this is huge waste of resources....
fn recursive_winning(card: &Card, cards: &Vec<Card>, card_count: &mut i32) {
    *card_count += 1;
    match card.get_matches() {
        0 => {}
        x => {
            let x: i32 = x as i32;
            cards[card.id as usize..(card.id + x) as usize]
                .into_iter()
                .for_each(|c| recursive_winning(c, &cards, card_count));
        }
    };
}

#[derive(Eq, PartialEq, Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card").field("id", &self.id).finish()
    }
}

impl Card {
    fn get_matches(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|x| self.your_numbers.contains(x))
            .count() as u32
    }

    fn get_points(&self) -> i32 {
        let base: i32 = 2;
        let matching_numbers = self.get_matches();

        match matching_numbers {
            0 => 0,
            1 => 1,
            x => base.pow(x - 1),
        }
    }
}

fn parse_card(line: &str) -> Card {
    match line.split(":").collect::<Vec<&str>>()[..] {
        [id, hands] => {
            let split: Vec<&str> = hands.split(" | ").collect();
            if split.len() != 2 {
                panic!("Invalid card")
            }

            Card {
                id: parse_id(id),
                winning_numbers: parse_numbers(split[0]),
                your_numbers: parse_numbers(split[1]),
            }
        }
        _ => panic!("Failed to parse card line"),
    }
}

fn parse_numbers(numbers: &str) -> Vec<i32> {
    numbers
        .trim()
        .split(" ")
        .filter(|&n| n.ne(""))
        .map(|n| match n.trim().parse::<i32>() {
            Ok(x) => x,
            Err(err) => panic!("Invalid number {}", err),
        })
        .collect()
}

const CARD_PREFIX: &'static str = "Card ";
fn parse_id(id_part: &str) -> i32 {
    match id_part.strip_prefix(CARD_PREFIX) {
        Some(id) => match id.trim().parse::<i32>() {
            Ok(id) => id,
            Err(err) => panic!("Failed to parse game id: {} {}", id, err),
        },
        None => panic!("Failed to parse game id"),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        let res = parse_id("Card 1");
        assert_eq!(1, res);

        let res = parse_id("Card 11");
        assert_eq!(11, res);

        let res = parse_id("Card 69");
        assert_eq!(69, res);
    }

    #[test]
    fn test_parse_numbers() {
        let res = parse_numbers("1 2 3 4 5");
        assert_eq!(vec![1, 2, 3, 4, 5], res);

        let res = parse_numbers("83 86  6 31 17  9 48 53");
        assert_eq!(vec![83, 86, 6, 31, 17, 9, 48, 53], res);
    }

    #[test]
    fn test_parse_card() {
        let res = parse_card("Card 1: 1 2 3 4 5 | 5 4 3 2 1");
        assert_eq!(
            Card {
                id: 1,
                winning_numbers: vec![1, 2, 3, 4, 5],
                your_numbers: vec![5, 4, 3, 2, 1],
            },
            res
        );
    }

    #[test]
    fn test_card_get_points() {
        let card = Card {
            id: 1,
            winning_numbers: vec![13, 32, 20, 16, 61],
            your_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        };

        assert_eq!(2, card.get_points());

        let card = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            your_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(8, card.get_points());
    }

    #[test]
    fn test_cards() {
        let res = cards(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            .to_string(),
        );

        assert_eq!(vec![8, 2, 2, 1, 0, 0], res);
    }

    #[test]
    fn test_pile() {
        let res = pile(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            .to_string(),
        );

        assert_eq!(30, res);
    }
}
