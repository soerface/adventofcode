use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_hand_type() != other.get_hand_type() {
            self.get_hand_type().cmp(&other.get_hand_type())
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl Hand {
    pub fn new(line: &str, with_joker: bool) -> Self {
        let mut cards = [Card::Two; 5];
        let mut i = 0;
        let mut split = line.split_whitespace();
        let card_str = split.next().unwrap();
        let bid = split.next().unwrap().parse().unwrap();
        for card in card_str.chars() {
            cards[i] = match card {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => if with_joker { Card::Joker } else { Card::Jack },
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("Unknown card: {}", card)
            };
            i += 1;
        }
        if i != 5 {
            panic!("Expected 5 cards, got {}", i);
        }
        Hand { cards, bid }
    }

    pub fn get_hand_type(&self) -> HandType {
        let mut counts = [0; 14];
        for card in self.cards.iter() {
            counts[*card as usize] += 1;
        }
        let joker_counts = counts[0];
        let mut pairs = 0;
        let mut threes = 0;
        let mut fours = 0;
        let mut fives = 0;
        for (i, count) in counts.iter().enumerate() {
            // skip jokers
            if i == 0 {
                continue;
            }
            if *count == 2 {
                pairs += 1;
            } else if *count == 3 {
                threes += 1;
            } else if *count == 4 {
                fours += 1;
            } else if *count == 5 {
                fives += 1;
            }
        }
        for _ in 0..joker_counts {
            if fours > 0 {
                fours -= 1;
                fives += 1;
            } else if threes > 0 {
                threes -= 1;
                fours += 1;
            } else if pairs > 0 {
                pairs -= 1;
                threes += 1;
            } else {
                pairs += 1;
            }
        }
        if fives == 1 {
            HandType::FiveOfAKind
        } else if fours == 1 {
            HandType::FourOfAKind
        } else if threes == 1 && pairs == 1 {
            HandType::FullHouse
        } else if threes == 1 {
            HandType::ThreeOfAKind
        } else if pairs == 2 {
            HandType::TwoPairs
        } else if pairs == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

pub fn stage1(file_path: &str) -> u64 {
    let input_str = read_to_string(file_path).unwrap();
    let mut hands: Vec<Hand> = input_str.lines().map(|line| Hand::new(line, false)).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| (i+1) as u64 * hand.bid).sum()
}

pub fn stage2(file_path: &str) -> u64 {
    let input_str = read_to_string(file_path).unwrap();
    let mut hands: Vec<Hand> = input_str.lines().map(|line| Hand::new(line, true)).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| (i+1) as u64 * hand.bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage1() {
        assert_eq!(stage1("inputs/day07/example.txt"), 6440);
        assert_eq!(stage1("inputs/day07/input.txt"), 245794640);
    }

    #[test]
    fn test_stage2() {
        assert_eq!(stage2("inputs/day07/example.txt"), 5905);
        assert_eq!(stage2("inputs/day07/input.txt"), 247899149);
    }

    #[test]
    fn compare() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Three < Card::Ten);
        assert!(Card::Two < Card::Ten);
        assert!(Hand::new("33332 1", false) > Hand::new("2AAAA 1", false));
        assert!(Hand::new("77888 1", false) > Hand::new("77788 1", false));
    }

    #[test]
    fn hand_types() {
        assert_eq!(Hand::new("T55J5 1", false).get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::new("KTJJT 1", false).get_hand_type(), HandType::TwoPairs);
        assert_eq!(Hand::new("QQQJA 1", false).get_hand_type(), HandType::ThreeOfAKind);

        assert_eq!(Hand::new("AAAAA 1", false).get_hand_type(), HandType::FiveOfAKind);
        assert_eq!(Hand::new("AA8AA 1", false).get_hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("23332 1", false).get_hand_type(), HandType::FullHouse);
        assert_eq!(Hand::new("TTT98 1", false).get_hand_type(), HandType::ThreeOfAKind);
        assert_eq!(Hand::new("23432 1", false).get_hand_type(), HandType::TwoPairs);
        assert_eq!(Hand::new("A23A4 1", false).get_hand_type(), HandType::OnePair);
        assert_eq!(Hand::new("23456 1", false).get_hand_type(), HandType::HighCard);
    }

    #[test]
    fn hand_types_with_joker() {
        // T55J5, KTJJT and QQQJA are now all four of a kind
        assert_eq!(Hand::new("T55J5 1", true).get_hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("KTJJT 1", true).get_hand_type(), HandType::FourOfAKind);
        assert_eq!(Hand::new("QQQJA 1", true).get_hand_type(), HandType::FourOfAKind);
    }
}