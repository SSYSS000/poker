use std::io::Write;

use itertools::Itertools;

struct Player {
    name: String,
    money: u32,
    hole_cards: Vec<Card>
}

impl Player {
    pub fn is_busted(&self) -> bool {
        self.money == 0
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum CardSuit {
    Spades,
    Hearts,
    Clubs,
    Diamonds
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum CardRank {
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
    Ace
}

#[derive(Debug)]
struct Card {
    suit: CardSuit,
    rank: CardRank
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: Vec::with_capacity(52) }
    }

    pub fn reset(&mut self) {
        let all_ranks = [
            CardRank::Two,
            CardRank::Three,
            CardRank::Four,
            CardRank::Five,
            CardRank::Six,
            CardRank::Seven,
            CardRank::Eight,
            CardRank::Nine,
            CardRank::Ten,
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
            CardRank::Ace
        ];

        let all_suits = [
            CardSuit::Hearts,
            CardSuit::Spades, 
            CardSuit::Clubs, 
            CardSuit::Diamonds
        ];

        self.cards.clear();

        for suit in all_suits {
            for rank in all_ranks {
                self.cards.push(Card { suit, rank });
            }
        }

    }

    pub fn shuffle(&mut self) {

    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Pot<'a> {
    size: u32,
    players: Vec<&'a Player>
}

struct Round<'a> {
    deck: Deck,
    community_cards: Vec<Card>,
    main_pot: Pot<'a>,
    side_pots: Vec<Pot<'a>>
}

struct Tournament {
    blinds: (u32, u32),
    players: Vec<Box<Player>>,
    dealer: u8
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush
}

fn determine_hand_value(cards: &mut [&Card]) -> HandRank {
    let cards: &mut [&Card; 5] = cards.try_into().expect("Hand is 5 cards.");

    // Sort cards by rank in descending order.
    cards.sort();
    cards.reverse();

    // All cards have same suit.
    let is_flush = cards
        .iter()
        .all(|card| card.suit == cards[0].suit);

    let is_straight = {
        cards
            .iter()
            .map(|card| card.rank as i32 - cards[4].rank as i32)
            .rev()
            .eq(0..5)
    };

    if is_flush {
        let is_royal = cards.iter()
            .all(|card| card.rank >= CardRank::Ten);

        if is_royal {
            HandRank::RoyalFlush
        }
        else if is_straight {
            HandRank::StraightFlush
        }
        else {
            HandRank::Flush
        }
    }
    else if is_straight {
        HandRank::Straight
    }
    else {
        let mut t: Vec<Vec<&&Card>> = Vec::new();

        // Group cards by rank.
        for (_, group) in &cards.iter().group_by(|&&card| card.rank) {
            t.push(group.collect());
        }

        // Sort groups by their lengths in descending order.
        t.sort_by_key(|k| k.len());
        t.reverse();

        match t[0].len() {
            4 => HandRank::FourOfAKind,

            3 => match t[1].len() {
                2 => HandRank::FullHouse,
                _ => HandRank::ThreeOfAKind
            },

            2 => match t[1].len() {
                2 => HandRank::TwoPair,
                _ => HandRank::Pair
            },

            _ => HandRank::HighCard
        }
    }
}

fn form_best_hand(community: &[Card], hole: &[Card])
{
    for h in hole.iter().combinations(2) {
        for mut c in community
        .iter()
        .chain(h.into_iter())
        .combinations(5) {

        }
    }
}

#[cfg(test)]
mod tests {
    use crate::determine_hand_value;
    use crate::{Card, CardRank, CardSuit, HandRank};

    #[test]
    fn hand_rankings() {
        let mut hand = [
            &Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Five },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Six },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ];

        assert_eq!(determine_hand_value(&mut hand), HandRank::StraightFlush);

        let mut hand = [
            &Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            &Card { suit: CardSuit::Diamonds, rank: CardRank::Four },
            &Card { suit: CardSuit::Spades, rank: CardRank::Seven },
            &Card { suit: CardSuit::Clubs, rank: CardRank::Seven },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ];

        assert_eq!(determine_hand_value(&mut hand), HandRank::ThreeOfAKind);

        let mut hand = [
            &Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            &Card { suit: CardSuit::Diamonds, rank: CardRank::Four },
            &Card { suit: CardSuit::Spades, rank: CardRank::Seven },
            &Card { suit: CardSuit::Clubs, rank: CardRank::Seven },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ];

        assert_eq!(determine_hand_value(&mut hand), HandRank::FullHouse);

        let mut hand = [
            &Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            &Card { suit: CardSuit::Diamonds, rank: CardRank::Five },
            &Card { suit: CardSuit::Spades, rank: CardRank::Nine },
            &Card { suit: CardSuit::Clubs, rank: CardRank::Jack },
            &Card { suit: CardSuit::Hearts, rank: CardRank::Two },
        ];

        assert_eq!(determine_hand_value(&mut hand), HandRank::HighCard);
    }

}

fn main() {
}
