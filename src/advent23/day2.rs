use std::{fs, path::Path};

pub fn run<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };

    return game(data).iter().sum();
}

fn game(input: String) -> Vec<i32> {
    let split = input.split('\n');
    split
        .into_iter()
        .filter(|&line| line.ne(""))
        .map(parse_game)
        .filter(Game::is_valid_game)
        .map(Game::get_id)
        .collect()
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };

    return power_of_cubes(data).iter().sum();
}

fn power_of_cubes(input: String) -> Vec<i32> {
    let split = input.split('\n');
    split
        .into_iter()
        .filter(|&line| line.ne(""))
        .map(parse_game)
        .map(Game::lowest_hand)
        .map(Hand::power)
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Game {
    id: i32,
    hands: Vec<Hand>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Hand {
    red: i32,
    green: i32,
    blue: i32,
}

impl Hand {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        Self { red, green, blue }
    }

    fn power(self) -> i32 {
        self.red * self.green * self.blue
    }
}

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

impl Game {
    // Valid when lt_eq then 12 red cubes, 13 green cubes, and 14 blue cubes
    fn is_valid_game(&self) -> bool {
        match self
            .hands
            .iter()
            .find(|hand| hand.red > RED || hand.green > GREEN || hand.blue > BLUE)
        {
            Some(_) => false,
            None => true,
        }
    }

    fn lowest_hand(self) -> Hand {
        let mut lowest_hand = Hand::default();
        for hand in self.hands.iter() {
            lowest_hand.red = lowest_hand.red.max(hand.red);
            lowest_hand.green = lowest_hand.green.max(hand.green);
            lowest_hand.blue = lowest_hand.blue.max(hand.blue);
        }

        lowest_hand
    }

    fn get_id(self) -> i32 {
        self.id
    }
}

fn parse_game(line: &str) -> Game {
    match line.split(":").collect::<Vec<&str>>()[..] {
        [id, hands] => Game {
            id: parse_id(id),
            hands: parse_hands(hands),
        },
        _ => panic!("Failed to parse game line"),
    }
}

fn parse_hands(hands_part: &str) -> Vec<Hand> {
    hands_part
        .trim()
        .split(";")
        .into_iter()
        .map(parse_hand)
        .collect()
}

fn parse_hand(hand_part: &str) -> Hand {
    let mut hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };

    let parts = hand_part.trim().split(",");
    for cubes in parts {
        let split: Vec<&str> = cubes.trim().split(" ").collect();
        if split.len() != 2 {
            panic!("Failed to parse cubes")
        }
        let count: i32 = split[0].parse::<i32>().unwrap();

        match split[1] {
            "green" => hand.green = count,
            "red" => hand.red = count,
            "blue" => hand.blue = count,
            color => panic!("Unknown color {}", color),
        }
    }

    hand
}

const GAME_PREFIX: &'static str = "Game ";

fn parse_id(id_part: &str) -> i32 {
    match id_part.strip_prefix(GAME_PREFIX) {
        Some(id) => id.parse::<i32>().unwrap(),
        None => panic!("Failed to parse game id"),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let res = parse_hand(" 2 green, 12 blue");
        assert_eq!(res, Hand::new(0, 2, 12));

        let res = parse_hand("6 red, 6 blue");
        assert_eq!(res, Hand::new(6, 0, 6));

        let res = parse_hand("8 blue, 5 green, 5 red");
        assert_eq!(res, Hand::new(5, 5, 8));
    }

    #[test]
    fn test_parse_hands() {
        let res = parse_hands(" 2 green, 12 blue; 6 red, 6 blue; 8 blue, 5 green, 5 red");
        assert_eq!(
            res,
            vec![Hand::new(0, 2, 12), Hand::new(6, 0, 6), Hand::new(5, 5, 8)]
        );
    }
    #[test]
    fn test_parse_id() {
        let res = parse_id("Game 1");
        assert_eq!(res, 1);

        let res = parse_id("Game 7");
        assert_eq!(res, 7);

        let res = parse_id("Game 12");
        assert_eq!(res, 12);
    }

    #[test]
    #[should_panic]
    fn test_parse_id_panic_1() {
        parse_id("Game k");
    }

    #[test]
    #[should_panic]
    fn test_parse_id_panic_2() {
        parse_id("Game");
    }

    #[test]
    fn test_game_is_valid() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(12, 13, 14)],
        };

        assert_eq!(true, game.is_valid_game());

        let game = Game {
            id: 1,
            hands: vec![Hand::new(2, 3, 4)],
        };

        assert_eq!(true, game.is_valid_game());

        let game = Game {
            id: 1,
            hands: vec![Hand::new(2, 3, 4), Hand::new(8, 9, 10)],
        };

        assert_eq!(true, game.is_valid_game());
    }

    #[test]
    fn test_game_is_invalid() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(21, 31, 41), Hand::new(1, 1, 1)],
        };

        assert_eq!(false, game.is_valid_game());

        let game = Game {
            id: 1,
            hands: vec![Hand::new(1, 1, 1), Hand::new(13, 14, 15)],
        };

        assert_eq!(false, game.is_valid_game());

        let game = Game {
            id: 1,
            hands: vec![Hand::new(1, 1, 1), Hand::new(1, 1, 16)],
        };

        assert_eq!(false, game.is_valid_game());
    }

    #[test]
    fn test_game_lowest_hand() {
        let game = Game {
            id: 1,
            hands: vec![Hand::new(1, 1, 1), Hand::new(1, 1, 13)],
        };

        assert_eq!(Hand::new(1, 1, 13), game.lowest_hand());

        let game = Game {
            id: 1,
            hands: vec![Hand::new(6, 2, 2), Hand::new(9, 8, 13)],
        };

        assert_eq!(Hand::new(9, 8, 13), game.lowest_hand());
    }

    #[test]
    fn test_hand_power() {
        let hand = Hand::new(1, 1, 13);
        assert_eq!(13, hand.power());

        let hand = Hand::new(5, 6, 2);
        assert_eq!(60, hand.power());

        let hand = Hand::new(6, 2, 2);
        assert_eq!(24, hand.power());
    }
}
