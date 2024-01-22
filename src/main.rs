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

impl std::fmt::Display for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CardSuit::*;

        match self {
            Spades   => '♠',
            Hearts   => '♥',
            Clubs    => '♣',
            Diamonds => '♦'
        }.fmt(f)
    }
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

impl std::fmt::Display for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CardRank::*;

        match self {
            Two   => "2",
            Three => "3",
            Four  => "4",
            Five  => "5",
            Six   => "6",
            Seven => "7",
            Eight => "8",
            Nine  => "9",
            Ten   => "10",
            Jack  => "J",
            Queen => "Q",
            King  => "K",
            Ace   => "A"
        }.fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
struct Card {
    suit: CardSuit,
    rank: CardRank
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {: >2}", self.suit, self.rank)
    }
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

            // Sort by group length and then card rank.
            t.sort_by(|a, b| match b.len().cmp(&a.len()) {
                std::cmp::Ordering::Equal => b[0].rank.cmp(&a[0].rank),
                o => o
            });

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

fn form_best_hand(community: &[Card], hole: &[Card]) -> Option<Hand>
{
    let mut hands: Vec<Hand> = Vec::new();

    for h in hole.iter().copied().combinations(2) {
        hands.push(
            community
            .iter()
            .copied()
            .chain(h.into_iter())
            .combinations(5)
            .map(|cards| Hand::new(cards.try_into().unwrap()))
            .max()
            .unwrap()
        );
    }

    hands.into_iter().max()
}

#[cfg(test)]
mod tests {
    use crate::{Card, CardRank, CardSuit, HandCategory, Hand};
    use CardRank::*;
    use CardSuit::*;
    use HandCategory::*;

    #[allow(non_snake_case)]
    const fn H(rank: CardRank) -> Card {
        Card { suit: Hearts, rank }
    }

    #[allow(non_snake_case)]
    const fn C(rank: CardRank) -> Card {
        Card { suit: Clubs, rank }
    }

    #[allow(non_snake_case)]
    const fn S(rank: CardRank) -> Card {
        Card { suit: Spades, rank }
    }

    #[allow(non_snake_case)]
    const fn D(rank: CardRank) -> Card {
        Card { suit: Diamonds, rank }
    }

    #[test]
    fn hand_categorization() {

        let hand = Hand::new([
            H(Jack),
            H(Ten),
            H(Ace),
            H(King),
            H(Queen),
        ]);

        assert_eq!(hand.category, RoyalFlush);

        let hand = Hand::new([
            H(Three),
            H(Four),
            H(Five),
            H(Six),
            H(Seven),
        ]);

        assert_eq!(hand.category, StraightFlush);

        let hand = Hand::new([
            H(Three),
            H(Two),
            H(Five),
            H(Ace),
            H(Seven),
        ]);

        assert_eq!(hand.category, Flush);

        let hand = Hand::new([
            D(Two),
            H(Jack),
            C(Two),
            S(Two),
            H(Two),
        ]);

        assert_eq!(hand.category, FourOfAKind);

        // Five-high straight.
        let hand = Hand::new([
            H(Ace),
            C(Four),
            S(Five),
            H(Three),
            H(Two),
        ]);

        assert_eq!(hand.category, Straight);

        // Ace-high straight.
        let hand = Hand::new([
            H(Jack),
            H(Ten),
            H(Ace),
            S(Queen),
            C(King),
        ]);

        assert_eq!(hand.category, Straight);

        // Eight-high straight.
        let hand = Hand::new([
            H(Eight),
            S(Six),
            H(Five),
            H(Four),
            C(Seven),
        ]);

        assert_eq!(hand.category, Straight);

        let hand = Hand::new([
            H(Three),
            D(Four),
            S(Seven),
            C(Seven),
            H(Seven),
        ]);

        assert_eq!(hand.category, ThreeOfAKind);

        let hand = Hand::new([
            H(Four),
            D(Four),
            S(Seven),
            C(Seven),
            H(Seven),
        ]);

        assert_eq!(hand.category, FullHouse);

        let hand = Hand::new([
            H(Four),
            D(Five),
            S(Five),
            C(Jack),
            H(Jack),
        ]);

        assert_eq!(hand.category, TwoPair);

        let hand = Hand::new([
            H(Four),
            D(Five),
            S(Nine),
            C(Jack),
            H(Jack),
        ]);

        assert_eq!(hand.category, Pair);

        let hand = Hand::new([
            H(Four),
            D(Five),
            S(Nine),
            C(Jack),
            H(Two),
        ]);

        assert_eq!(hand.category, HighCard);
    }

    #[test]
    fn hand_comparison() {
        let king_high = Hand::new([
            H(Four),
            D(Five),
            S(Three),
            C(King),
            H(Two),
        ]);

        let jack_high = Hand::new([
            H(Four),
            D(Five),
            S(Nine),
            C(Jack),
            H(Two),
        ]);

        assert!(jack_high < king_high);
    }

}

fn main() {
}
