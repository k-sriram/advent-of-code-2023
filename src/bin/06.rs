use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(6);

lazy_static! {
    static ref TIME_RE: Regex = Regex::new(r"Time:\s+(\d+(?:\s+\d+)*)").unwrap();
    static ref DISTANCE_RE: Regex = Regex::new(r"Distance:\s+(\d+(?:\s+\d+)*)").unwrap();
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_input_one(input)?;
    races
        .iter()
        .map(|r| num_better_times(&r))
        .product::<i64>()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_input_two(input)?;
    num_better_times(&race).try_into().ok()
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn delay(&self) -> (f64, f64) {
        let t = self.time as f64;
        let d = self.distance as f64;
        let c = (t.powi(2) - 4.0 * d).sqrt();
        ((t - c) / 2.0, (t + c) / 2.0)
    }
}

fn num_better_times(race: &Race) -> i64 {
    let (t1, t2) = race.delay();
    let (t1i, t2i) = ((t1 + 1.0).floor() as i64, (t2 - 1.0).ceil() as i64);
    std::cmp::max(0, t2i - t1i + 1)
}

fn parse_input_one(input: &str) -> Option<Vec<Race>> {
    let time = TIME_RE
        .captures(input)?
        .get(1)?
        .as_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap());
    let distance = DISTANCE_RE
        .captures(input)?
        .get(1)?
        .as_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap());

    Some(
        time.zip(distance)
            .map(|(t, d)| Race {
                time: t,
                distance: d,
            })
            .collect(),
    )
}

fn parse_input_two(input: &str) -> Option<Race> {
    let time = parse_re_number(input, &TIME_RE)?;
    let distance = parse_re_number(input, &DISTANCE_RE)?;
    Some(Race { time, distance })
}

fn parse_re_number(input: &str, re: &Regex) -> Option<u64> {
    re.captures(input)?
        .get(1)?
        .as_str()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
