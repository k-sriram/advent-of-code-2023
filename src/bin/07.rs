advent_of_code::solution!(7);

pub use one::part as part_one;
pub use two::part as part_two;

mod one {
    use phf::phf_map;
    static CARD_VALUE_STD: phf::Map<char, u8> = phf_map!(
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14
    );

    pub fn part(input: &str) -> Option<u32> {
        let mut bids = parse_input(input)?;
        bids.sort_by_key(|bid| bid.hand);
        let mut winnings = 0;
        for (i, bid) in bids.iter().enumerate() {
            // println!("{}: {} {:?}", i + 1, bid, bid.hand.type_of());
            winnings += bid.bid * (i as u32 + 1);
        }
        Some(winnings)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Card(u8);

    impl Card {
        fn from_char(card: char) -> Option<Self> {
            CARD_VALUE_STD.get(&card).copied().map(Self)
        }
    }

    impl std::fmt::Display for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let card = match self.0 {
                2..=9 => self.0.to_string(),
                10 => "T".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                _ => unreachable!(),
            };
            write!(f, "{}", card)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Hand([Card; 5]);

    impl Hand {
        fn from_str(hand: &str) -> Option<Self> {
            if hand.len() != 5 {
                return None;
            }
            hand.chars()
                .map(Card::from_char)
                .collect::<Option<Vec<_>>>()
                .map(|cards| Self(cards.try_into().unwrap()))
        }

        fn type_of(&self) -> HandType {
            let mut cards = self.0;
            cards.sort_unstable_by_key(|card| card.0);
            let mut counts = [0; 13];
            for card in cards.iter() {
                counts[(card.0 - 2) as usize] += 1;
            }
            let mut counts = counts.iter().copied().collect::<Vec<_>>();
            counts.sort_unstable();
            counts.reverse();
            match counts.as_slice() {
                [5, ..] => HandType::FiveOfAKind,
                [4, ..] => HandType::FourOfAKind,
                [3, 2, ..] => HandType::FullHouse,
                [3, 1, ..] => HandType::ThreeOfAKind,
                [2, 2, 1, ..] => HandType::TwoPair,
                [2, 1, 1, 1, ..] => HandType::Pair,
                _ => HandType::HighCard,
            }
        }
    }

    impl std::fmt::Display for Hand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let cards = self
                .0
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<_>>()
                .join("");
            write!(f, "{}", cards)
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            std::cmp::Ord::cmp(&(self.type_of(), self.0), &(other.type_of(), other.0))
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    struct Bid {
        hand: Hand,
        bid: u32,
    }

    impl Bid {
        fn from_str(bid: &str) -> Option<Self> {
            let mut bid = bid.split_whitespace();
            let hand = bid.next()?;
            let bid = bid.next()?;
            let bid = bid.parse().ok()?;
            Some(Self {
                hand: Hand::from_str(hand)?,
                bid,
            })
        }
    }

    impl std::fmt::Display for Bid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.hand, self.bid)
        }
    }

    fn parse_input(input: &str) -> Option<Vec<Bid>> {
        input.lines().map(Bid::from_str).collect::<Option<Vec<_>>>()
    }
}

mod two {
    use phf::phf_map;
    static CARD_VALUE_STD: phf::Map<char, u8> = phf_map!(
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14
    );

    pub fn part(input: &str) -> Option<u32> {
        let mut bids = parse_input(input)?;
        bids.sort_by_key(|bid| bid.hand);
        let mut winnings = 0;
        for (i, bid) in bids.iter().enumerate() {
            // println!("{}: {} {:?}", i + 1, bid, bid.hand.type_of());
            winnings += bid.bid * (i as u32 + 1);
        }
        Some(winnings)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Card(u8);

    impl Card {
        fn from_char(card: char) -> Option<Self> {
            CARD_VALUE_STD.get(&card).copied().map(Self)
        }
    }

    impl std::fmt::Display for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let card = match self.0 {
                2..=9 => self.0.to_string(),
                10 => "T".to_string(),
                1 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                _ => unreachable!(),
            };
            write!(f, "{}", card)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum HandType {
        HighCard,
        Pair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Hand([Card; 5]);

    impl Hand {
        fn from_str(hand: &str) -> Option<Self> {
            if hand.len() != 5 {
                return None;
            }
            hand.chars()
                .map(Card::from_char)
                .collect::<Option<Vec<_>>>()
                .map(|cards| Self(cards.try_into().unwrap()))
        }

        fn type_of(&self) -> HandType {
            let mut cards = self.0;
            cards.sort_unstable_by_key(|card| card.0);
            let mut counts = [0; 13];
            let mut jokers = 0;
            for card in cards.iter() {
                if card.0 == 1 {
                    jokers += 1;
                    continue;
                }
                counts[(card.0 - 2) as usize] += 1;
            }
            let mut counts = counts.iter().copied().collect::<Vec<_>>();
            counts.sort_unstable();
            counts.reverse();
            let result = match counts.as_slice() {
                [5, ..] => HandType::FiveOfAKind,
                [4, ..] => HandType::FourOfAKind,
                [3, 2, ..] => HandType::FullHouse,
                [3, ..] => HandType::ThreeOfAKind,
                [2, 2, ..] => HandType::TwoPair,
                [2, ..] => HandType::Pair,
                _ => HandType::HighCard,
            };
            promote_type(result, jokers)
        }
    }

    fn promote_type(result: HandType, jokers: u32) -> HandType {
        use HandType::*;
        match (result, jokers) {
            (_, 0) => result,
            (HighCard, 1) => Pair,
            (Pair, 1) => ThreeOfAKind,
            (TwoPair, 1) => FullHouse,
            (ThreeOfAKind, 1) => FourOfAKind,
            (FourOfAKind, 1) => FiveOfAKind,
            (HighCard, 2) => ThreeOfAKind,
            (Pair, 2) => FourOfAKind,
            (ThreeOfAKind, 2) => FiveOfAKind,
            (HighCard, 3) => FourOfAKind,
            (Pair, 3) => FiveOfAKind,
            (HighCard, 4) => FiveOfAKind,
            (HighCard, 5) => FiveOfAKind,
            _ => unreachable!("{:?} cannot have {} jokers", result, jokers),
        }
        
    }

    impl std::fmt::Display for Hand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let cards = self
                .0
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<_>>()
                .join("");
            write!(f, "{}", cards)
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            std::cmp::Ord::cmp(&(self.type_of(), self.0), &(other.type_of(), other.0))
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug)]
    struct Bid {
        hand: Hand,
        bid: u32,
    }

    impl Bid {
        fn from_str(bid: &str) -> Option<Self> {
            let mut bid = bid.split_whitespace();
            let hand = bid.next()?;
            let bid = bid.next()?;
            let bid = bid.parse().ok()?;
            Some(Self {
                hand: Hand::from_str(hand)?,
                bid,
            })
        }
    }

    impl std::fmt::Display for Bid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {}", self.hand, self.bid)
        }
    }

    fn parse_input(input: &str) -> Option<Vec<Bid>> {
        input.lines().map(Bid::from_str).collect::<Option<Vec<_>>>()
    }
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
}
