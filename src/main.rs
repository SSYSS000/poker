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

#[derive(Clone, Copy, Debug)]
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
enum HandCategory {
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    category: HandCategory,
    // The order of 'cards' is significant in comparing the ranks of two hands.
    // The card(s) that define the hand category come first and in descending
    // rank order. If there are one or more kickers, they follow the
    // aforementioned cards in descending rank order.
    // The sorting method ranks ace lowest if doing so results
    // in a stronger hand (e.g. a "five-high straight" over an "ace high").
    cards: [Card; 5]
}

impl Hand {
    pub fn new(mut cards: [Card; 5]) -> Hand {
        Hand {
            category: Self::sort_and_categorize(&mut cards),
            cards: cards
        }
    }

    fn sort_and_categorize(cards: &mut [Card; 5]) -> HandCategory {
        cards.sort();
        cards.reverse();

        // All cards have same suit.
        let is_flush = cards.iter()
            .all(|card| card.suit == cards[0].suit);

        let is_straight = {
            let mut sub = [0i8; 4];

            // Compare the 4 highest cards to the lowest ranking card.
            for i in 0..sub.len() {
                sub[i] = cards[i].rank as i8 - cards[4].rank as i8;
            }

            if sub.eq(&[4, 3, 2, 1]) {
                true
            }
            else if sub.eq(&[12, 3, 2, 1]) {
                // Five-high straight.
                cards.rotate_left(1);
                true
            }
            else {
                false
            }
        };

        if is_flush {
            let is_royal = cards.iter()
                .all(|card| card.rank >= CardRank::Ten);

            if is_royal {
                HandCategory::RoyalFlush
            }
            else if is_straight {
                HandCategory::StraightFlush
            }
            else {
                HandCategory::Flush
            }
        }
        else if is_straight {
            HandCategory::Straight
        }
        else {
            let mut t: Vec<Vec<Card>> = Vec::with_capacity(5);

            // Group cards by rank.
            for (_, group) in &(*cards).into_iter().group_by(|card| card.rank) {
                t.push(group.collect());
            }

            // Sort groups by their lengths in descending order.
            t.sort_by_key(|k| k.len());
            t.reverse();

            // Copy the order.
            for (i, &card) in t.iter().flatten().enumerate() {
                cards[i] = card;
            }

            match t[0].len() {
                4 => HandCategory::FourOfAKind,

                3 => match t[1].len() {
                    2 => HandCategory::FullHouse,
                    _ => HandCategory::ThreeOfAKind
                },

                2 => match t[1].len() {
                    2 => HandCategory::TwoPair,
                    _ => HandCategory::Pair
                },

                _ => HandCategory::HighCard
            }
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
    use crate::{Card, CardRank, CardSuit, HandCategory, Hand};

    #[test]
    fn hand_rankings() {
        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Jack },
            Card { suit: CardSuit::Hearts, rank: CardRank::Ten },
            Card { suit: CardSuit::Hearts, rank: CardRank::Ace },
            Card { suit: CardSuit::Hearts, rank: CardRank::King },
            Card { suit: CardSuit::Hearts, rank: CardRank::Queen },
        ]);

        assert_eq!(hand.category, HandCategory::RoyalFlush);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Hearts, rank: CardRank::Five },
            Card { suit: CardSuit::Hearts, rank: CardRank::Six },
            Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ]);

        assert_eq!(hand.category, HandCategory::StraightFlush);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            Card { suit: CardSuit::Hearts, rank: CardRank::Two },
            Card { suit: CardSuit::Hearts, rank: CardRank::Five },
            Card { suit: CardSuit::Hearts, rank: CardRank::Ace },
            Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ]);

        assert_eq!(hand.category, HandCategory::Flush);

        let hand = Hand::new([
            Card { suit: CardSuit::Diamonds, rank: CardRank::Two },
            Card { suit: CardSuit::Hearts, rank: CardRank::Jack },
            Card { suit: CardSuit::Clubs, rank: CardRank::Two },
            Card { suit: CardSuit::Spades, rank: CardRank::Two },
            Card { suit: CardSuit::Hearts, rank: CardRank::Two },
        ]);

        assert_eq!(hand.category, HandCategory::FourOfAKind);

        // Five-high straight.
        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Ace },
            Card { suit: CardSuit::Clubs, rank: CardRank::Four },
            Card { suit: CardSuit::Spades, rank: CardRank::Five },
            Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            Card { suit: CardSuit::Hearts, rank: CardRank::Two },
        ]);

        assert_eq!(hand.category, HandCategory::Straight);

        // Ace-high straight.
        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Jack },
            Card { suit: CardSuit::Hearts, rank: CardRank::Ten },
            Card { suit: CardSuit::Hearts, rank: CardRank::Ace },
            Card { suit: CardSuit::Spades, rank: CardRank::Queen },
            Card { suit: CardSuit::Clubs, rank: CardRank::King },
        ]);

        assert_eq!(hand.category, HandCategory::Straight);

        // Eight-high straight.
        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Eight },
            Card { suit: CardSuit::Spades, rank: CardRank::Six },
            Card { suit: CardSuit::Hearts, rank: CardRank::Five },
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Clubs, rank: CardRank::Seven },
        ]);

        assert_eq!(hand.category, HandCategory::Straight);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Three },
            Card { suit: CardSuit::Diamonds, rank: CardRank::Four },
            Card { suit: CardSuit::Spades, rank: CardRank::Seven },
            Card { suit: CardSuit::Clubs, rank: CardRank::Seven },
            Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ]);

        assert_eq!(hand.category, HandCategory::ThreeOfAKind);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Diamonds, rank: CardRank::Four },
            Card { suit: CardSuit::Spades, rank: CardRank::Seven },
            Card { suit: CardSuit::Clubs, rank: CardRank::Seven },
            Card { suit: CardSuit::Hearts, rank: CardRank::Seven },
        ]);

        assert_eq!(hand.category, HandCategory::FullHouse);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Diamonds, rank: CardRank::Five },
            Card { suit: CardSuit::Spades, rank: CardRank::Five },
            Card { suit: CardSuit::Clubs, rank: CardRank::Jack },
            Card { suit: CardSuit::Hearts, rank: CardRank::Jack },
        ]);

        assert_eq!(hand.category, HandCategory::TwoPair);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Diamonds, rank: CardRank::Five },
            Card { suit: CardSuit::Spades, rank: CardRank::Nine },
            Card { suit: CardSuit::Clubs, rank: CardRank::Jack },
            Card { suit: CardSuit::Hearts, rank: CardRank::Jack },
        ]);

        assert_eq!(hand.category, HandCategory::Pair);

        let hand = Hand::new([
            Card { suit: CardSuit::Hearts, rank: CardRank::Four },
            Card { suit: CardSuit::Diamonds, rank: CardRank::Five },
            Card { suit: CardSuit::Spades, rank: CardRank::Nine },
            Card { suit: CardSuit::Clubs, rank: CardRank::Jack },
            Card { suit: CardSuit::Hearts, rank: CardRank::Two },
        ]);

        assert_eq!(hand.category, HandCategory::HighCard);
    }

}

fn main() {
}
