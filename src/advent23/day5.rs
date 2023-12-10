use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{collections::HashMap, fs, path::Path};

pub fn run<P: AsRef<Path>>(path: P) -> i64 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };

    return seeds(data);
}

fn seeds(input: String) -> i64 {
    let almanac = parse(input);

    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_location(seed))
        .min()
        .or(Some(0))
        .unwrap()
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> i64 {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(err) => panic!("Failed to load input file: {}", err.to_string()),
    };

    return seeds_range(data);
}

fn seeds_range(input: String) -> i64 {
    let almanac = parse(input);

    let locations: Vec<_> = almanac
        .seeds
        .chunks(2)
        .map(|seed_range| {
            let range = seed_range[0]..seed_range[0] + seed_range[1];

            range
                .par_bridge()
                .progress_count(seed_range[1] as u64)
                .map(|seed| almanac.seed_to_location(seed))
                .min()
        })
        .collect();

    locations
        .into_iter()
        .map(|v| v.or(Some(0)).unwrap())
        .min()
        .or(Some(0))
        .unwrap()
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: SeedMap,
    soil_to_fertilizer: SeedMap,
    fertilizer_to_water: SeedMap,
    water_to_light: SeedMap,
    light_to_temperature: SeedMap,
    temperature_to_huminidity: SeedMap,
    huminidity_to_location: SeedMap,
}

impl Almanac {
    fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let huminidity = self.temperature_to_huminidity.get(temperature);
        self.huminidity_to_location.get(huminidity)
    }
}

#[derive(PartialEq, Eq, Debug, Default)]
struct SeedMap {
    map: Vec<(i64, i64, i64)>,
}

impl SeedMap {
    fn new(map: Vec<(i64, i64, i64)>) -> Self {
        Self { map }
    }
}

impl SeedMap {
    fn get(&self, i: i64) -> i64 {
        let maybe_match = self.map.iter().find(|&m| m.1 <= i && m.1 + m.2 > i);
        match maybe_match {
            Some(m) => i + m.0 - m.1,
            None => i,
        }
    }
}

// What is this monstrosity ?
impl From<&str> for SeedMap {
    fn from(value: &str) -> Self {
        let map = value
            .trim()
            .split("\n")
            .into_iter()
            .map(parse_map_line)
            .collect();

        SeedMap { map }
    }
}

fn parse_map_line(line: &str) -> (i64, i64, i64) {
    let numbers: Vec<i64> = line
        .split(" ")
        .filter(|&n| n.ne(""))
        .into_iter()
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect();

    if numbers.len() != 3 {
        panic!("Invalid map line {}", line);
    }

    (numbers[0], numbers[1], numbers[2])
}

fn parse_seeds(seeds: &str) -> Vec<i64> {
    seeds
        .trim()
        .split(" ")
        .into_iter()
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

fn parse(input: String) -> Almanac {
    let chunks = input.trim().split("\n\n");
    chunks.fold(Almanac::default(), |mut acc, chunk| {
        let chunk_split: Vec<&str> = chunk.split(":").collect();
        if chunk_split.len() != 2 {
            panic!("Invalid chunk {}", chunk);
        };

        if chunk_split[0].starts_with("seeds") {
            acc.seeds = parse_seeds(chunk_split[1]);
        };

        if chunk_split[0].starts_with("seed-to-soil") {
            acc.seed_to_soil = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("soil-to-fertilizer") {
            acc.soil_to_fertilizer = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("fertilizer-to-water") {
            acc.fertilizer_to_water = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("water-to-light") {
            acc.water_to_light = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("light-to-temperature") {
            acc.light_to_temperature = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("temperature-to-humidity") {
            acc.temperature_to_huminidity = SeedMap::from(chunk_split[1]);
        };

        if chunk_split[0].starts_with("humidity-to-location") {
            acc.huminidity_to_location = SeedMap::from(chunk_split[1]);
        };

        acc
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_seeds() {
        let res = seeds(
            "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
                        "
            .to_string(),
        );

        assert_eq!(35, res);
    }

    #[test]
    fn test_seedmap_from_string() {
        let res = SeedMap::from("50 98 2");

        assert_eq!(SeedMap::new(vec![(50, 98, 2)]), res);
    }

    #[test]
    fn test_seedmap_get() {
        let res = SeedMap::from("52 50 48").get(1);
        assert_eq!(res, 1);

        let res = SeedMap::from("52 50 48").get(79);
        assert_eq!(res, 81);
    }

    #[test]
    fn test_parse_seeds() {
        let res = parse_seeds("50 98 2");
        assert_eq!(vec![50, 98, 2], res);
    }

    #[test]
    fn test_parse() {
        let res = parse(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2

soil-to-fertilizer map:
0 15  1

fertilizer-to-water map:
49 53 1

water-to-light map:
88 18 1

light-to-temperature map:
45 77 1

temperature-to-humidity map:
0 69 1

humidity-to-location map:
56 93 1"
                .to_string(),
        );

        assert_eq!(
            Almanac {
                seeds: vec![79, 14, 55, 13],
                seed_to_soil: SeedMap::new(vec![(50, 98, 2)]),
                soil_to_fertilizer: SeedMap::new(vec![(0, 15, 1)]),
                fertilizer_to_water: SeedMap::new(vec![(49, 53, 1)]),
                water_to_light: SeedMap::new(vec![(88, 18, 1)]),
                light_to_temperature: SeedMap::new(vec![(45, 77, 1)]),
                temperature_to_huminidity: SeedMap::new(vec![(0, 69, 1)]),
                huminidity_to_location: SeedMap::new(vec![(56, 93, 1)]),
            },
            res
        );
    }

    #[test]
    fn test_almanac_seed_to_location() {
        let almanac = parse(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
                .to_string(),
        );

        assert_eq!(81, almanac.seed_to_soil.get(79));
        assert_eq!(81, almanac.soil_to_fertilizer.get(81));
        assert_eq!(81, almanac.fertilizer_to_water.get(81));
        assert_eq!(74, almanac.water_to_light.get(81));
        assert_eq!(78, almanac.light_to_temperature.get(74));
        assert_eq!(78, almanac.temperature_to_huminidity.get(78));
        assert_eq!(82, almanac.huminidity_to_location.get(78));

        assert_eq!(82, almanac.seed_to_location(79));
    }
}
