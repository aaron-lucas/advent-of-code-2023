use crate::challenge::DailyChallenge;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Day7;

#[derive(Eq, Ord, Hash, PartialEq, PartialOrd, Clone, Debug)]
enum CamelCard {
    Joker,
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl CamelCard {
    fn from_char(ch: char, use_jokers: bool) -> Result<Self, String> {
        match ch {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => {
                if use_jokers {
                    Ok(Self::Joker)
                } else {
                    Ok(Self::Jack)
                }
            }
            'T' => Ok(Self::Number(10)),
            _ => {
                if let Some(digit) = ch.to_digit(10) {
                    Ok(Self::Number(digit as u8))
                } else {
                    Err(format!("Cannot convert {ch} to CamelCard"))
                }
            }
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[CamelCard; 5]) -> Self {
        let mut counts: HashMap<&CamelCard, u8> = HashMap::new();
        for card in cards {
            counts
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1u8);
        }

        // Handle the effects of Jokers later
        let mut count_vals_no_jokers: Vec<u8> = counts
            .iter()
            .filter_map(|(&k, &v)| {
                if let CamelCard::Joker = k {
                    None
                } else {
                    Some(v)
                }
            })
            .collect();

        count_vals_no_jokers.sort();
        count_vals_no_jokers.reverse();
        let mut top_2: Vec<u8> = count_vals_no_jokers.iter().take(2).cloned().collect();

        if let Some(&num_jokers) = counts.get(&CamelCard::Joker) {
            if num_jokers == 5 {
                return HandType::FiveOfAKind;
            } else {
                top_2[0] = top_2[0] + num_jokers;
            }
        }

        match top_2[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1] => HandType::ThreeOfAKind,
            [2, 2] => HandType::TwoPair,
            [2, 1] => HandType::Pair,
            [1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [CamelCard; 5],
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(cards: [CamelCard; 5], bid: u32) -> Self {
        let hand_type = HandType::from_cards(&cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }

    fn vec_from_file(file: &Path, use_jokers: bool) -> Result<Vec<Self>, String> {
        let contents = fs::read_to_string(file).map_err(|e| e.to_string())?;

        contents
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let cards: [CamelCard; 5] = chars
                    .by_ref()
                    .take(5)
                    .map(|c| CamelCard::from_char(c, use_jokers))
                    .collect::<Result<Vec<CamelCard>, String>>()?
                    .try_into()
                    .unwrap();

                let bid: u32 = chars
                    .skip(1)
                    .collect::<String>()
                    .parse::<u32>()
                    .map_err(|e| e.to_string())?;

                Ok(Hand::new(cards, bid))
            })
            .collect()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card != other_card {
                    return self_card.cmp(other_card);
                }
            }

            std::cmp::Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl DailyChallenge for Day7 {
    fn part1(&self, file: &Path) -> u64 {
        let mut hands = Hand::vec_from_file(&file, false).unwrap();
        hands.sort();
        hands
            .iter()
            .zip(1..)
            .map(|(hand, rank)| hand.bid * rank)
            .sum::<u32>() as u64
    }

    fn part2(&self, file: &Path) -> u64 {
        let mut hands = Hand::vec_from_file(&file, true).unwrap();
        hands.sort();
        hands
            .iter()
            .zip(1..)
            .map(|(hand, rank)| hand.bid * rank)
            .sum::<u32>() as u64
    }
}

#[test]
fn test_part1() {
    assert_eq!(Day7.part1(Path::new("data/7.sample")), 6440);
}

#[test]
fn test_part2() {
    assert_eq!(Day7.part2(Path::new("data/7.sample")), 5905);
}
