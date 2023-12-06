use std::{fs, path::Path};

pub fn run<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return parser(data).iter().sum();
}

fn parser(input: String) -> Vec<i32> {
    let split = input.split('\n');
    split
        .into_iter()
        .filter(|&line| line.ne(""))
        .map(parse_digit_pair)
        .map(pair_to_number)
        .collect()
}

fn parse_digit_pair(line: &str) -> (Option<u32>, Option<u32>) {
    let mut numbers = line
        .chars()
        .into_iter()
        .filter(|x| x.is_numeric())
        .map(|x| x.to_digit(10).unwrap());

    (numbers.next(), numbers.last())
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return parser_with_words(data).iter().sum();
}

fn parser_with_words(input: String) -> Vec<i32> {
    let split = input.split('\n');
    split
        .into_iter()
        .filter(|&line| line.ne(""))
        .map(replace_words)
        .map(parse_digit_pair)
        .map(|pair| {
            let first = pair.0.unwrap_or(0);
            let last = pair.1.unwrap_or(first);
            first as i32 * 10 + last as i32
        })
        .collect()
}

fn pair_to_number(pair: (Option<u32>, Option<u32>)) -> i32 {
    let first = match pair.0 {
        Some(n) => n,
        None => pair.1.unwrap_or(0),
    };
    let last = pair.1.unwrap_or(first);
    first as i32 * 10 + last as i32
}

const WORDS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const DIGITS: [&'static str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

// This was almost elegant, but wrong for case "eightwo"
fn replace_words(line: &str) -> &str {
    let min_size = 3;
    if line.len() < min_size {
        return line;
    }

    let max_size = 5;
    let mut current_size = 3;

    let mut result = String::new();

    let mut chars = line.to_string();
    let mut buffer: String = chars[0..current_size].to_string();
    loop {
        if buffer.len() == 0 {
            break;
        }

        let maybe_match = WORDS
            .into_iter()
            .enumerate()
            .find(|(_, word)| buffer.eq(word));

        if maybe_match.is_some() {
            result.push_str(DIGITS[maybe_match.unwrap().0]);
            if current_size <= chars.len() {
                chars = chars[1..].to_string();
                current_size = min_size;
                buffer = chars[0..current_size.min(chars.len())].to_string();
            }
            continue;
        }

        if chars.len() == 0 {
            let removed = buffer.pop();
            match removed {
                Some(removed) => {
                    result.push(removed);
                }
                None => {
                    break;
                }
            }
            continue;
        }

        current_size += 1;
        if current_size > max_size {
            current_size = min_size;
            let removed = chars.remove(0);
            result.push(removed);
        }
        if current_size > chars.len() {
            current_size = chars.len() - 1;
            let removed = chars.remove(0);
            result.push(removed);
        }
        buffer = chars[0..current_size].to_string();
    }

    result.leak()
}

mod tests {
    use super::*;

    #[test]
    fn test_parser_oneliner() {
        let res = parser("1abc2\n".to_string());
        assert_eq!(res, vec![12])
    }

    #[test]
    fn test_parser_multiline() {
        let res = parser("pqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string());
        assert_eq!(res, vec![38, 15, 77])
    }

    #[test]
    fn test_parse_digit_pair() {
        let res = parse_digit_pair("pqr3stu8vwx");
        assert_eq!(res, (Some(3), Some(8)));

        let res = parse_digit_pair("pqr3stuvwx");
        assert_eq!(res, (Some(3), None));

        let res = parse_digit_pair("pqrstuvwx");
        assert_eq!(res, (None, None));
    }

    #[test]
    fn test_replace_words() {
        let res = replace_words("eightwo");
        assert_eq!(res, "8igh2wo");

        let res = replace_words("two1nine");
        assert_eq!(res, "2wo19ine");

        let res = replace_words("eightwothree");
        assert_eq!(res, "8igh2wo3hree");

        let res = replace_words("twone");
        assert_eq!(res, "2w1ne");

        let res = replace_words("823");
        assert_eq!(res, "823");

        let res = replace_words("3k");
        assert_eq!(res, "3k");
    }

    #[test]
    fn test_pair_to_number() {
        let res = pair_to_number((Some(8), Some(2)));
        assert_eq!(res, 82);

        let res = pair_to_number((Some(8), None));
        assert_eq!(res, 88);

        let res = pair_to_number((None, None));
        assert_eq!(res, 0);

        let res = pair_to_number((None, Some(6)));
        assert_eq!(res, 66);
    }

    #[test]
    fn test_parser_with_words_multiline() {
        let res = parser_with_words(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                .to_string(),
        );
        assert_eq!(res, vec![29, 83, 13, 24, 42, 14, 76])
    }

    #[test]
    fn test_parser_with_words_multiline_2() {
        let res = parser_with_words(
            "
ninefive7cnxznfmcp6nine
eight4one9x3nine
eightnine4kgxhxx1ckrqlrn
6sevenkjmfxrbhck
jsgtwonefvmcdsnqfp4fivefivesevenhkbkqcb1vgkshfnxfc
eightcvzmtlvsm49
78four
threeoneninecjzs75
xrxrsrh58
1zqkhcvoneseventwohbrfbqgvp
9zfznrfvtgjfhsk
v5jcblbstnvxk
6s1
6three1seven
75xpmzmhqqphgtrblhkcdxczcvbmg
"
            .to_string(),
        );
        assert_eq!(
            res,
            vec![99, 89, 81, 67, 21, 89, 74, 35, 58, 12, 99, 55, 61, 67, 75]
        )
    }

    #[test]
    fn test_parser_with_words_multiline_with_sum() {
        let res = parser_with_words(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                .to_string(),
        );
        assert_eq!(res.iter().sum::<i32>(), 281)
    }
}
