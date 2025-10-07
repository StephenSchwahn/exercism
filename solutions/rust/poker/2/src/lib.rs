use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl<'a> From<&'a str> for Suit {
    fn from(value: &'a str) -> Self {
        match value {
            "H" => Self::Heart,
            "D" => Self::Diamond,
            "S" => Self::Spade,
            "C" => Self::Club,
            _ => panic!("inconceivable"),
        }
    }
}

impl<'a> From<&'a str> for Rank {
    fn from(value: &'a str) -> Self {
        match value {
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::Four,
            "5" => Self::Five,
            "6" => Self::Six,
            "7" => Self::Seven,
            "8" => Self::Eight,
            "9" => Self::Nine,
            "10" => Self::Ten,
            "J" => Self::J,
            "Q" => Self::Q,
            "K" => Self::K,
            "A" => Self::A,
            _ => panic!("inconceivable"),
        }
    }
}

#[derive(Eq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl<'a> From<&'a str> for Card {
    fn from(value: &'a str) -> Self {
        // len should be 2 chars (or 3 in the case of "10H")
        let (rank_str, suit_str) = value.split_at(value.len() - 1);
        Card {
            rank: Rank::from(rank_str),
            suit: Suit::from(suit_str),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank // Suit is irrelevant in most poker derivatives
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlushData<'a> {
    ranks: Vec<&'a Rank>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StraightData<'a> {
    high_card: &'a Card,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HighCardData<'a> {
    ranks: Vec<&'a Rank>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TripsData<'a> {
    high_card: &'a Rank,
    rest: Vec<&'a Rank>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuadsData<'a> {
    high_card: &'a Rank,
    kicker: &'a Rank,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullHouseData<'a> {
    trips_rank: &'a Rank,
    pairs_rank: &'a Rank,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPairData<'a> {
    larger_pair: &'a Rank,
    smaller_pair: &'a Rank,
    single_value: &'a Rank,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PairData<'a> {
    pair: &'a Rank,
    rest: Vec<&'a Rank>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandStrength<'a> {
    HighCard(HighCardData<'a>),
    Pair(PairData<'a>),
    TwoPair(TwoPairData<'a>), // Tie breaks on the higher of the two ranks
    Trips(TripsData<'a>),
    Straight(StraightData<'a>),
    Flush(FlushData<'a>),
    FullHouse(FullHouseData<'a>), // Whichever one has three of a kind that's higher. This should work with the automatic ordering
    FourOfAKind(QuadsData<'a>),
    StraightFlush(StraightData<'a>),
}

impl<'a> HandStrength<'a> {
    fn is_flush(cards: &'a [Card; 5]) -> Option<FlushData<'a>> {
        if cards.iter().all(|card| card.suit == cards[0].suit) {
            let rank_sorted = cards
                .into_iter()
                .sorted_by_key(|card| card.rank as u8)
                .rev()
                .map(|card| &card.rank)
                .collect::<Vec<&Rank>>();
            Some(FlushData {
                ranks: rank_sorted
            })
        } else {
            None
        }
    }

    fn is_straight(cards: &[Card; 5]) -> Option<StraightData> {
        let mut consecutive_count = 1;
        let sorted: Vec<&Card> = cards
            .into_iter()
            .sorted_by_key(|card| card.rank as u8)
            .collect();
        let mut prev_value = sorted[0].rank as u8;

        for i in 1..5 {
            let rank_value = sorted[i].rank as u8;
            if rank_value - prev_value == 1 {
                consecutive_count += 1;
            }
            prev_value = rank_value;
        }

        if consecutive_count == 5 {
            sorted.last().map(|&card| StraightData { high_card: card })
        } else if consecutive_count == 4 && sorted[3].rank == Rank::Five && sorted[4].rank == Rank::A
        {
            // Account for ace being low card
            sorted
                .iter()
                .nth(3)
                .map(|&card| StraightData { high_card: card })
        } else {
            None
        }
    }

    fn groups_by_ranks(cards: &[Card; 5]) -> HashMap<&Rank, Vec<&Card>> {
        cards
            .iter()
            .sorted_by_key(|card| card.rank as u8)
            .rev() // put big numbers first
            .group_by(|card| &card.rank)
            .into_iter()
            .map(|(rank, group)| (rank, group.collect::<Vec<_>>()))
            .collect::<HashMap<&Rank, Vec<&Card>>>()
    }

    fn is_quads(cards: &HashMap<&'a Rank, Vec<&'a Card>>) -> Option<QuadsData<'a>> {
        let quads_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 4)
            .map(|(rank, _)| rank )?;
        
        let kicker_rank: &'a Rank = cards
            .iter()
            .filter(|(_, group)| group.len() == 1)
            .map(|(&rank, _)| rank)
            .last()?;

        Some(QuadsData {
            high_card: &quads_rank,
            kicker: kicker_rank
        })
    }

    fn is_full_house(cards: &HashMap<&'a Rank, Vec<&'a Card>>) -> Option<FullHouseData<'a>> {
        let trips_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 3)
            .map(|(rank, _)| rank)?;
        let pairs_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 2)
            .map(|(rank, _)| rank)?;
        Some(FullHouseData {
            pairs_rank,
            trips_rank,
        })
    }

    fn is_trips(cards: &HashMap<&'a Rank, Vec<&'a Card>>) -> Option<TripsData<'a>> {
        let pairs_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 3)
            .map(|(rank, _)| rank)?;

        let rest_ranks: Vec<&'a Rank> = cards
            .iter()
            .filter(|(rank, _)| *rank != pairs_rank)
            .filter(|(_, group)| group.len() == 1)
            .sorted()
            .rev()
            .map(|(&rank, _)| rank)
            .collect();

        if rest_ranks.len() != 2 {
            None
        } else {
            Some(TripsData {
                high_card: pairs_rank,
                rest: rest_ranks,
            })
        }
    }

    fn is_two_pair(cards: &HashMap<&'a Rank, Vec<&'a Card>>) -> Option<TwoPairData<'a>> {
        let pairs_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 2)
            .map(|(rank, _)| rank)?;
        let other_pairs_rank = cards
            .iter()
            .filter(|(rank, _)| *rank != pairs_rank)
            .find(|(_, group)| group.len() == 2)
            .map(|(rank, _)| rank)?;

        let single_value_rank = cards
            .iter()
            .filter(|(rank, _)| *rank != pairs_rank && *rank != other_pairs_rank)
            .take(1)
            .map(|(rank, _)| rank)
            .last()?;

        Some(TwoPairData {
            larger_pair: if pairs_rank > other_pairs_rank {
                pairs_rank
            } else {
                other_pairs_rank
            },
            smaller_pair: if pairs_rank > other_pairs_rank {
                other_pairs_rank
            } else {
                pairs_rank
            },
            single_value: single_value_rank,
        })
    }

    fn is_pair(cards: &HashMap<&'a Rank, Vec<&'a Card>>) -> Option<PairData<'a>> {
        let pairs_rank = cards
            .iter()
            .find(|(_, group)| group.len() == 2)
            .map(|(rank, _)| rank)?;

        let rest_ranks: Vec<&'a Rank> = cards
            .iter()
            .filter(|(rank, _)| *rank != pairs_rank)
            .filter(|(_, group)| group.len() == 1)
            .sorted()
            .rev()
            .map(|(&rank, _)| rank)
            .collect();

        if rest_ranks.len() != 3 {
            None
        } else {
            Some(PairData {
                pair: pairs_rank,
                rest: rest_ranks,
            })
        }
    }
}

impl<'a> From<&'a [Card; 5]> for HandStrength<'a> {
    fn from(cards: &'a [Card; 5]) -> Self {
        let flush = HandStrength::is_flush(&cards);
        let straight = HandStrength::is_straight(&cards);
        let rank_groupings = HandStrength::groups_by_ranks(&cards);
        let four_of_a_kind = HandStrength::is_quads(&rank_groupings);
        let full_house = HandStrength::is_full_house(&rank_groupings);
        let trips = HandStrength::is_trips(&rank_groupings);
        let two_pair = HandStrength::is_two_pair(&rank_groupings);
        let pair = HandStrength::is_pair(&rank_groupings);

        if let (Some(straight), Some(_)) = (&straight, &flush) {
            HandStrength::StraightFlush(StraightData {
                high_card: straight.high_card,
            })
        } else if let Some(four_of_a_kind) = four_of_a_kind {
            HandStrength::FourOfAKind(four_of_a_kind)
        } else if let Some(full_house) = full_house {
            HandStrength::FullHouse(full_house)
        } else if let Some(flush) = flush {
            HandStrength::Flush(flush)
        } else if let Some(straight) = &straight {
            HandStrength::Straight(StraightData {
                high_card: straight.high_card,
            })
        } else if let Some(trips) = trips {
            HandStrength::Trips(trips)
        } else if let Some(two_pairs) = two_pair {
            HandStrength::TwoPair(two_pairs)
        } else if let Some(pair) = pair {
            HandStrength::Pair(pair)
        } else {
            HandStrength::HighCard(HighCardData {
                ranks: rank_groupings
                    .keys()
                    .sorted()
                    .rev()
                    .map(|key| *key)
                    .collect::<Vec<&Rank>>(),
            })
        }
    }
}

#[derive(Eq, Debug)]
pub struct PokerHand<'a> {
    pub str_ref: &'a str,
    pub cards: [Card; 5],
}

impl<'a> From<&'a str> for PokerHand<'a> {
    fn from(value: &'a str) -> Self {
        let cards: Vec<Card> = value
            .split_whitespace()
            .map(|s| Card::from(s))
            .take(5)
            .collect();

        let cards: [Card; 5] = cards.try_into().unwrap_or_else(|v: Vec<Card>| {
            panic!("Expected a Vec of length {} but it was {}", 5, v.len())
        });

        PokerHand {
            str_ref: value,
            cards: cards,
        }
    }
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        let hand_str = HandStrength::from(&self.cards);
        let other_hand_str = HandStrength::from(&other.cards);

        hand_str == other_hand_str
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_str = HandStrength::from(&self.cards);
        let other_hand_str = HandStrength::from(&other.cards);

        hand_str.cmp(&other_hand_str)
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let sorted_hands: Vec<PokerHand<'a>> = hands
        .iter()
        .map(|&hand_str| PokerHand::from(hand_str))
        .sorted()
        .rev()
        .collect();

    let equal_vecs = sorted_hands
        .iter()
        .take_while(|hand| **hand == sorted_hands[0])
        .map(|hand| hand.str_ref)
        .collect();

    equal_vecs
}
