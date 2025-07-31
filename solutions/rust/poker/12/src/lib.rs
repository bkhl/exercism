use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum CardRank {
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Card {
    pub rank: CardRank,
    pub suit: Suit,
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s.chars().next().unwrap() {
            '2' => CardRank::Two,
            '3' => CardRank::Three,
            '4' => CardRank::Four,
            '5' => CardRank::Five,
            '6' => CardRank::Six,
            '7' => CardRank::Seven,
            '8' => CardRank::Eight,
            '9' => CardRank::Nine,
            '1' => CardRank::Ten,
            'J' => CardRank::Jack,
            'Q' => CardRank::Queen,
            'K' => CardRank::King,
            'A' => CardRank::Ace,
            _ => return Err(ParseCardError {}),
        };

        let suit = match s.chars().last().unwrap() {
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => return Err(ParseCardError {}),
        };

        Ok(Card { suit, rank })
    }
}

#[derive(Debug)]
pub struct ParseCardError {}

/// Representation of a poker hand.
pub struct Hand(HashSet<Card>);

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().map(|c| c.parse()).collect() {
            Ok(cards) => Ok(Hand(cards)),
            Err(_) => Err(ParseHandError {}),
        }
    }
}

#[derive(Debug)]
pub struct ParseHandError {}

// Given an array of string slices, return any of the same same string slices
// that represent winning hands.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    struct HandInfo<'a> {
        string: &'a str,
        hand_rank: HandRank,
        card_values: Vec<i8>,
    }

    let mut result = Vec::<&str>::new();

    let all_hands: Vec<HandInfo> = hands
        .iter()
        .map(|&s| {
            let hand: Hand = s.parse().unwrap();
            let (hand_rank, card_values) = get_hand_rank(&hand);
            HandInfo {
                string: &s,
                hand_rank,
                card_values,
            }
        }).collect();

    let mut hand_iterator = all_hands.into_iter();
    let mut previous;

    if let Some(first) = hand_iterator.next() {
        result.push(first.string);
        previous = first;

        for current in hand_iterator {
            match current.hand_rank.cmp(&previous.hand_rank) {
                Ordering::Greater => {
                    result.clear();
                    result.push(current.string);
                    previous = current;
                }
                Ordering::Equal => match current.card_values.cmp(&previous.card_values) {
                    Ordering::Greater => {
                        result.clear();
                        result.push(current.string);
                        previous = current;
                    }
                    Ordering::Equal => {
                        result.push(current.string);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Some(result)
}

// Given a `Hand`, returns a `HandRank`, and a `Vec<i8>` of significant card
// values for purposes of comparing the hand to other hands of the same rank.
fn get_hand_rank(hand: &Hand) -> (HandRank, Vec<i8>) {
    let mut card_rank_count: HashMap<i8, i8> = HashMap::new();

    for card in hand.0.clone() {
        *card_rank_count.entry(card.rank as i8).or_insert(0) += 1;
    }

    let mut card_values: Vec<i8> = card_rank_count
        .clone()
        .into_iter()
        .map(|(k, _)| k as i8)
        .collect();

    card_values.sort_by(|a, b| match card_rank_count[&b].cmp(&card_rank_count[a]) {
        Ordering::Equal => b.cmp(a),
        ordering => ordering,
    });

    let straight = if card_values.len() == 5 {
        let high_card = card_values.first().cloned().unwrap();
        let low_card = card_values.last().cloned().unwrap();

        if high_card == low_card + 4 {
            card_values.truncate(1);
            true
        } else if high_card == CardRank::Ace as i8 && card_values.get(1).cloned().unwrap() == CardRank::Five as i8 {
            card_values.clear();
            card_values.push(CardRank::Two as i8 - 1);
            true
        } else {
            false
        }
    } else {
        false
    };

    let mut card_iterator = hand.0.iter();
    let first_suit = card_iterator.next().cloned().unwrap().suit;
    let flush = card_iterator.all(|c| c.suit == first_suit);

    let rank_option = match (straight, flush) {
        (true, false) => Some(HandRank::Straight),
        (false, true) => Some(HandRank::Flush),
        (true, true) => Some(HandRank::StraightFlush),
        _ => None,
    };

    if let Some(rank) = rank_option {
        return (rank, card_values);
    }

    let max_count = *card_rank_count.values().max().unwrap();

    let rank = if max_count == 4 {
        HandRank::FourOfAKind
    } else if card_rank_count.len() == 2 {
        HandRank::FullHouse
    } else if max_count == 3 {
        HandRank::ThreeOfAKind
    } else if max_count == 2 {
        let pair_count = card_rank_count
            .iter()
            .filter(|(_, &count)| count == 2)
            .count();

        match pair_count {
            2 => HandRank::TwoPairs,
            1 => HandRank::Pair,
            _ => unreachable!(),
        }
    } else {
        HandRank::HighCard
    };

    (rank, card_values)
}
