use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

pub fn run<P: AsRef<Path>>(path: P) -> i32 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return lift(data).iter().sum();
}

// I'm not really happy with this since it's kinda messy, but it's single iteration.
fn lift(input: String) -> Vec<i32> {
    let schema = parse(input);
    let schema_len = schema.len() - 1;

    let mut result = Vec::new();
    let mut buffer = Vec::new();
    for (r, row) in schema.clone().into_iter().enumerate() {
        let r: i32 = r as i32;
        let row_len = row.len() - 1;
        let mut row_iter = row.clone().into_iter().enumerate();
        let mut is_part_number = false;
        loop {
            let (c, char) = match row_iter.next() {
                Some(t) => t,
                None => break,
            };
            let c: i32 = c as i32;
            if char.is_numeric() {
                let r_bounder = |x: i32| x.max(0).min(schema_len as i32) as usize;
                let c_bounder = |x: i32| x.max(0).min(row_len as i32) as usize;
                let tl = schema[r_bounder(r - 1)][c_bounder(c - 1)];
                let tm = schema[r_bounder(r - 1)][c_bounder(c)];
                let tr = schema[r_bounder(r - 1)][c_bounder(c + 1)];

                let ml = schema[r_bounder(r)][c_bounder(c - 1)];
                let mr = schema[r_bounder(r)][c_bounder(c + 1)];

                let bl = schema[r_bounder(r + 1)][c_bounder(c - 1)];
                let bm = schema[r_bounder(r + 1)][c_bounder(c)];
                let br = schema[r_bounder(r + 1)][c_bounder(c + 1)];

                let has_magic_around = is_magic_char(tl)
                    || is_magic_char(tm)
                    || is_magic_char(tr)
                    || is_magic_char(ml)
                    || is_magic_char(mr)
                    || is_magic_char(bl)
                    || is_magic_char(bm)
                    || is_magic_char(br);

                is_part_number = is_part_number || has_magic_around;
                buffer.push(char);
            } else {
                if !buffer.is_empty() && is_part_number {
                    let id: i32 = match buffer.iter().cloned().collect::<String>().parse() {
                        Ok(id) => id,
                        Err(err) => panic!("We matched something we didn't want to: {}", err),
                    };
                    result.push(id);
                }
                buffer.clear();
                is_part_number = false;
            }
        }

        if !buffer.is_empty() && is_part_number {
            let id: i32 = match buffer.iter().cloned().collect::<String>().parse() {
                Ok(id) => id,
                Err(err) => panic!("We matched something we didn't want to: {}", err),
            };
            result.push(id);
        }
        buffer.clear();
    }

    return result;
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> i64 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };
    return gears(data).iter().sum();
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

// I'm not really happy with this since it's kinda messy, but it's single iteration.
// This is smacking my ass, cannot figure out where is the mistake
fn gears(input: String) -> Vec<i64> {
    let schema = parse(input);
    let schema_len = schema.len() - 1;

    let mut result: HashMap<Coordinate, Vec<i64>> = HashMap::new();
    let mut buffer = Vec::new();
    for (r, row) in schema.clone().into_iter().enumerate() {
        let r: i32 = r as i32;
        let row_len = row.len() - 1;
        let mut row_iter = row.clone().into_iter().enumerate();
        let mut has_gear_attached = false;
        let mut gear_positions = HashSet::new();
        loop {
            let (c, char) = match row_iter.next() {
                Some(t) => t,
                None => break,
            };
            let c: i32 = c as i32;
            if char.is_numeric() {
                let r_bounder = |x: i32| x.max(0).min(schema_len as i32) as usize;
                let c_bounder = |x: i32| x.max(0).min(row_len as i32) as usize;
                let tl = schema[r_bounder(r - 1)][c_bounder(c - 1)];
                if is_gear(tl) {
                    gear_positions.insert(Coordinate::new(r_bounder(r - 1), c_bounder(c - 1)));
                }
                let tm = schema[r_bounder(r - 1)][c_bounder(c)];
                if is_gear(tm) {
                    gear_positions.insert(Coordinate::new(r_bounder(r - 1), c_bounder(c)));
                }
                let tr = schema[r_bounder(r - 1)][c_bounder(c + 1)];
                if is_gear(tr) {
                    gear_positions.insert(Coordinate::new(r_bounder(r - 1), c_bounder(c + 1)));
                }

                let ml = schema[r_bounder(r)][c_bounder(c - 1)];
                if is_gear(ml) {
                    gear_positions.insert(Coordinate::new(r_bounder(r), c_bounder(c - 1)));
                }
                let mr = schema[r_bounder(r)][c_bounder(c + 1)];
                if is_gear(mr) {
                    gear_positions.insert(Coordinate::new(r_bounder(r), c_bounder(c + 1)));
                }

                let bl = schema[r_bounder(r + 1)][c_bounder(c - 1)];
                if is_gear(bl) {
                    gear_positions.insert(Coordinate::new(r_bounder(r + 1), c_bounder(c - 1)));
                }
                let bm = schema[r_bounder(r + 1)][c_bounder(c)];
                if is_gear(bm) {
                    gear_positions.insert(Coordinate::new(r_bounder(r + 1), c_bounder(c)));
                }
                let br = schema[r_bounder(r + 1)][c_bounder(c + 1)];
                if is_gear(br) {
                    gear_positions.insert(Coordinate::new(r_bounder(r + 1), c_bounder(c + 1)));
                }

                let has_gear_around = is_gear(tl)
                    || is_gear(tm)
                    || is_gear(tr)
                    || is_gear(ml)
                    || is_gear(mr)
                    || is_gear(bl)
                    || is_gear(bm)
                    || is_gear(br);

                has_gear_attached = has_gear_attached || has_gear_around;
                buffer.push(char);
            } else {
                if !buffer.is_empty() && has_gear_attached {
                    let id: i64 = match buffer.iter().cloned().collect::<String>().parse() {
                        Ok(id) => id,
                        Err(err) => panic!("We matched something we didn't want to: {}", err),
                    };
                    for position in gear_positions.clone().into_iter() {
                        let mut ids = match result.get(&position) {
                            Some(v) => v.clone(),
                            None => Vec::new(),
                        };
                        ids.push(id);
                        result.insert(position, ids);
                    }
                }
                buffer.clear();
                has_gear_attached = false;
                gear_positions.clear();
            }
        }

        if !buffer.is_empty() && has_gear_attached {
            let id: i64 = match buffer.iter().cloned().collect::<String>().parse() {
                Ok(id) => id,
                Err(err) => panic!("We matched something we didn't want to: {}", err),
            };

            for position in gear_positions.into_iter() {
                let mut ids = match result.get(&position) {
                    Some(v) => v.clone(),
                    None => Vec::new(),
                };
                ids.push(id);
                result.insert(position, ids);
            }
        }
        buffer.clear();
    }

    return result
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .collect();
}

fn is_magic_char(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .filter(|line| line.to_owned().ne(""))
        .map(|line| line.trim().chars().collect())
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn test_lift() {
        let res = lift(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .to_string(),
        );
        assert_eq!(vec![467, 35, 633, 617, 592, 755, 664, 598], res);

        let res = lift(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .to_string(),
        );
        assert_eq!(4361, res.iter().sum());

        let res = lift(
            "
467#.114..
..........
.#35..633#
..........
617*......
.......58.
..592+....
......755.
.&*$......
.664..598#"
                .to_string(),
        );
        assert_eq!(vec![467, 35, 633, 617, 592, 755, 664, 598], res);

        let res = lift(
            "
467#...4#.
..........
.#35..633#
..........
617*......
.......58.
..592+....
......755.
.&*$......
.664..598#"
                .to_string(),
        );
        assert_eq!(vec![467, 4, 35, 633, 617, 592, 755, 664, 598], res);

        let res = lift(
            "
467#....#4
..........
.#35..633#
..........
617*......
.......58.
..592+....
......755.
.&*$......
.664..598#"
                .to_string(),
        );
        assert_eq!(vec![467, 4, 35, 633, 617, 592, 755, 664, 598], res);
    }

    #[test]
    fn test_gears() {
        let mut res = gears(
            "
467..114..
....*.....
..35...140
........*.
617...925.
.....+.58*
..592...&.
......755.
...$.&....
.664.598.."
                .to_string(),
        );

        res.sort();
        assert_eq!(vec![114 * 35, 925 * 58, 140 * 925], res);
    }
}
