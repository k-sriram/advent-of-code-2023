use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_cards(input);
    cards.iter().map(|c| c.calc_points()).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input);
    let winners = cards
        .iter()
        .map(|c| c.get_winning_numbers().len())
        .collect::<Vec<_>>();
    let mut num_cards = vec![1; cards.len()];
    for i in 0..cards.len() {
        for j in i + 1..std::cmp::min(i + winners[i] + 1, cards.len()) {
            num_cards[j] += num_cards[i];
        }
    }
    Some(num_cards.iter().sum())
}

lazy_static! {
    static ref CARD_RE: Regex =
        Regex::new(r"Card\s+(\d+):(\s+(?:\d+\s+)+)\|(\s+(?:\d+\s*)+)").unwrap();
}

struct Card {
    id: u32,
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_str(line: &str) -> Self {
        let caps = CARD_RE.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winners = caps
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let numbers = caps
            .get(3)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Self {
            id,
            winners,
            numbers,
        }
    }

    fn is_winner(&self, number: u32) -> bool {
        self.winners.contains(&number)
    }

    fn get_winning_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|n| self.is_winner(**n))
            .map(|n| *n)
            .collect()
    }

    fn calc_points(&self) -> u32 {
        let num_win = self.get_winning_numbers().len();
        if num_win == 0 {
            0
        } else {
            let mut points = 1;
            for _ in 1..num_win {
                points *= 2;
            }
            points
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("id", &self.id)
            .field("winners", &self.winners)
            .field("numbers", &self.numbers)
            .finish()
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(|line| Card::from_str(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
