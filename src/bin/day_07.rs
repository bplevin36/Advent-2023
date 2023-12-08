use aoc2023::read_input;
use enum_map::{Enum, enum_map};
use nom::{IResult, multi::separated_list1, character::complete::{newline, u32 as parse_u32, multispace1}, sequence::separated_pair, bytes::complete::take};

use std::{cmp::Ordering, time::Instant};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Enum)]
#[repr(u8)]
enum Card {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    J,
    Q,
    K,
    A,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'A' => Card::A,
            b'K' => Card::K,
            b'Q' => Card::Q,
            b'J' => Card::J,
            b'T' => Card::TEN,
            b'9' => Card::NINE,
            b'8' => Card::EIGHT,
            b'7' => Card::SEVEN,
            b'6' => Card::SIX,
            b'5' => Card::FIVE,
            b'4' => Card::FOUR,
            b'3' => Card::THREE,
            b'2' => Card::TWO,
            _ => panic!("Invalid card char: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand{
    cards: [Card; 5],
    hand_type: HandType,
}

impl HandType {
    fn from_cards(cards: [Card; 5]) -> HandType {
        let mut card_counts = enum_map! {
            Card::TWO => (Card::TWO, 0u32),
            Card::THREE => (Card::THREE, 0u32),
            Card::FOUR => (Card::FOUR, 0u32),
            Card::FIVE => (Card::FIVE, 0u32),
            Card::SIX => (Card::SIX, 0u32),
            Card::SEVEN => (Card::SEVEN, 0u32),
            Card::EIGHT => (Card::EIGHT, 0u32),
            Card::NINE => (Card::NINE, 0u32),
            Card::TEN => (Card::TEN, 0u32),
            Card::J => (Card::J, 0u32),
            Card::Q => (Card::Q, 0u32),
            Card::K => (Card::K, 0u32),
            Card::A => (Card::A, 0u32),
        };
        for &card in cards.iter() {
            card_counts[card].1 += 1;
        }
        let counts_list = card_counts.as_mut_slice();
        counts_list.sort_by_key(|(_, v)| *v);
        match counts_list {
            [.., (_, 5)] => HandType::FiveKind,
            [.., (_, 4)] => HandType::FourKind,
            [.., (_, 2), (_, 3)] => HandType::FullHouse,
            [.., (_, 3)] => HandType::ThreeKind,
            [.., (_, 2), (_, 2)] => HandType::TwoPair,
            [.., (_, 2)] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl Hand {
    pub fn from_input(input: &str) -> IResult<&str, Hand> {
        let (i, cards_str) = take(5usize)(input)?;
        let mut cards = [Card::A; 5];
        let cards_bytes = cards_str.as_bytes();
        for (&byte, card_to_set) in cards_bytes.iter().zip(cards.iter_mut()) {
            let card: Card = byte.into();
            *card_to_set = card;
        }
        let hand_type = HandType::from_cards(cards);
        Ok((i, Hand {
            cards,
            hand_type,
        }))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);
        if type_order != Ordering::Equal {
            type_order
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hands_and_bids(input: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(newline, separated_pair(Hand::from_input, multispace1, parse_u32))(input)
}
pub fn main() {
    let start_time = Instant::now();
    let input = read_input("07");

    let (_, mut hands_bids) = parse_hands_and_bids(&input).unwrap();
    hands_bids.sort_by(|(hand, _), (hand2, _)| hand.cmp(hand2));

    let mut sum = 0u64;
    for ((_hand, bid), rank) in hands_bids.into_iter().zip(1..) {
        let winnings = bid as u64 * rank as u64;
        sum += winnings;
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
