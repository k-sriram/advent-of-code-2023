use phf::phf_map;

static DIGIT_STR: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(p1_parse_line)
        .reduce(option_sum)
        .unwrap_or(None)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(p2_parse_line)
        .reduce(option_sum)
        .unwrap_or(None)
}

fn option_sum(a: Option<u32>, b: Option<u32>) -> Option<u32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    }
}

fn p1_parse_line(line: &str) -> Option<u32> {
    let first = p1_first_digit(line);
    let last = p1_last_digit(line);
    match (first, last) {
        (Some(first), Some(last)) => Some(10 * first + last),
        _ => None,
    }
}

fn p1_first_digit(line: &str) -> Option<u32> {
    let chars = line.chars();
    p1_first_digit_iter(chars)
}

fn p1_last_digit(line: &str) -> Option<u32> {
    let chars = line.chars().rev();
    p1_first_digit_iter(chars)
}

fn p1_first_digit_iter<I>(chars: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    for c in chars {
        if c.is_digit(10) {
            return c.to_digit(10);
        }
    }
    None
}

fn p2_parse_line(line: &str) -> Option<u32> {
    let first = p2_first_digit(line);
    let last = p2_last_digit(line);
    match (first, last) {
        (Some(first), Some(last)) => Some(10 * first + last),
        _ => None,
    }
}

fn p2_first_digit(line: &str) -> Option<u32> {
    for i in 1..line.len() + 1 {
        for j in [1, 3, 4, 5] {
            if j > i {
                continue;
            }
            let substr = &line[i - j..i];
            let digit = parse_digit(substr);
            if digit.is_some() {
                return digit;
            }
        }
    }
    None
}

fn p2_last_digit(line: &str) -> Option<u32> {
    for i in (0..line.len()).rev() {
        for j in [1, 3, 4, 5] {
            if j + i > line.len() {
                continue;
            }
            let substr = &line[i..i + j];
            let digit = parse_digit(substr);
            if digit.is_some() {
                return digit;
            }
        }
    }
    None
}

fn parse_digit(text: &str) -> Option<u32> {
    if text.len() == 1 {
        text.chars().next().unwrap().to_digit(10)
    } else {
        DIGIT_STR.get(text).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_parse_digits() {
        assert_eq!(parse_digit("one"), Some(1));
        assert_eq!(parse_digit("3"), Some(3));
        assert_eq!(parse_digit("ten"), None);
        assert_eq!(parse_digit("13"), None);
    }

    #[test]
    fn test_p2_digit() {
        assert_eq!(p2_first_digit("psblg3"), Some(3));
        assert_eq!(p2_last_digit("psblg3"), Some(3));
        assert_eq!(p2_first_digit("4nineeightseven2"), Some(4));
    }
}
