advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let histories = parse_input(input);
    Some(
        histories
            .iter()
            .map(|h| find_next(h))
            .sum::<i64>()
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let histories = parse_input(input);
    Some(
        histories
            .iter()
            .map(|h| find_prev(h))
            .sum::<i64>()
    )
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_next(history: &[i64]) -> i64 {
    if history.iter().all(|&n| n == 0) {
        return 0;
    }
    history.last().unwrap() + find_next(&diff(history))
}

fn find_prev(history: &[i64]) -> i64 {
    if history.iter().all(|&n| n == 0) {
        return 0;
    }
    history.first().unwrap() - find_prev(&diff(history))
}


fn diff(history: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 0..history.len() - 1 {
        result.push(history[i + 1] - history[i]);
    }
    result
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
