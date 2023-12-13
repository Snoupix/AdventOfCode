use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::{collections::HashMap, str::FromStr};

use crate::{AdventOfCode, Day};

pub struct Seven;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardScore {
    Nothing,
    HighCard(CardType),
    Pair(CardType),
    TwoPair(CardType, CardType),
    ThreeOfAKind(CardType),
    FullHouse(CardType, CardType),
    FourOfAKind(CardType),
    FiveOfAKind(CardType),
}

impl Ord for CardScore {
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Greater | Equal => self,
            Less => other,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Self::Nothing => {
                if matches!(other, Self::Nothing) {
                    Equal
                } else {
                    Less
                }
            }
            Self::HighCard(c) => {
                if matches!(other, Self::Nothing) {
                    Greater
                } else if let Self::HighCard(_c) = other {
                    c.cmp(_c)
                } else {
                    Less
                }
            }
            Self::Pair(c) => match other {
                Self::Nothing | Self::HighCard(_) => Greater,
                Self::Pair(_c) => c.cmp(_c),
                _ => Less,
            },
            Self::TwoPair(c1, c2) => match other {
                Self::Nothing | Self::HighCard(_) | Self::Pair(_) => Greater,
                Self::TwoPair(_c1, _c2) => c1.max(c2).cmp(_c1.max(_c2)),

                _ => Less,
            },
            Self::ThreeOfAKind(c) => match other {
                Self::Nothing | Self::HighCard(_) | Self::Pair(_) | Self::TwoPair(_, _) => Greater,
                Self::ThreeOfAKind(_c) => c.cmp(_c),
                _ => Less,
            },
            Self::FullHouse(c1, c2) => match other {
                Self::FourOfAKind(_) | Self::FiveOfAKind(_) => Less,
                Self::FullHouse(_c1, _c2) => c1.max(c2).cmp(_c1.max(_c2)),
                _ => Greater,
            },
            Self::FourOfAKind(c) => match other {
                Self::FiveOfAKind(_) => Less,
                Self::FourOfAKind(_c) => c.cmp(_c),
                _ => Greater,
            },
            Self::FiveOfAKind(c) => {
                if let Self::FiveOfAKind(_c) = other {
                    c.cmp(_c)
                } else {
                    Greater
                }
            }
        }
    }
}

impl PartialOrd for CardScore {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Less)
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Less | Equal)
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Greater)
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Greater | Equal)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

impl Ord for CardType {
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.cmp(&other) {
            Greater | Equal => self,
            Less => other,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Ace => {
                if *other == Self::Ace {
                    Equal
                } else {
                    Greater
                }
            }
            Self::King => {
                if matches!(other, Self::Ace) {
                    Less
                } else if matches!(other, Self::King) {
                    Equal
                } else {
                    Greater
                }
            }
            Self::Queen => match other {
                Self::Ace | Self::King => Less,
                Self::Queen => Equal,
                _ => Greater,
            },
            Self::Jack => match other {
                Self::Ace | Self::King | Self::Queen => Less,
                Self::Jack => Equal,
                _ => Greater,
            },
            Self::Ten => match other {
                Self::Ace | Self::King | Self::Queen | Self::Jack => Less,
                Self::Ten => Equal,
                _ => Greater,
            },
            Self::Nine => match other {
                Self::Ace | Self::King | Self::Queen | Self::Jack | Self::Ten => Less,
                Self::Nine => Equal,
                _ => Greater,
            },
            Self::Eight => match other {
                Self::Ace | Self::King | Self::Queen | Self::Jack | Self::Ten | Self::Nine => Less,

                Self::Eight => Equal,
                _ => Greater,
            },
            Self::Seven => match other {
                Self::Six | Self::Five | Self::Four | Self::Three | Self::Two | Self::One => {
                    Greater
                }
                Self::Seven => Equal,
                _ => Less,
            },
            Self::Six => match other {
                Self::Five | Self::Four | Self::Three | Self::Two | Self::One => Greater,
                Self::Six => Equal,
                _ => Less,
            },
            Self::Five => match other {
                Self::Four | Self::Three | Self::Two | Self::One => Greater,
                Self::Five => Equal,
                _ => Less,
            },
            Self::Four => match other {
                Self::Three | Self::Two | Self::One => Greater,
                Self::Four => Equal,
                _ => Less,
            },
            Self::Three => match other {
                Self::Two | Self::One => Greater,
                Self::Three => Equal,
                _ => Less,
            },
            Self::Two => match other {
                Self::One => Greater,
                Self::Two => Equal,
                _ => Less,
            },
            Self::One => {
                if matches!(other, Self::One) {
                    Equal
                } else {
                    Less
                }
            }
        }
    }
}

impl PartialOrd for CardType {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Less)
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Less | Equal)
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Greater)
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Greater | Equal)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for CardType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Jack,
            "T" => Self::Ten,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            "1" => Self::One,
            t => panic!("Invalid card type {t}"),
        })
    }
}

impl CardScore {
    fn get_score(cards: Vec<CardType>) -> Self {
        let mut nb_of: HashMap<CardType, u32> = HashMap::new();
        let mut pair_nb = Vec::new();
        let mut three_nb = Vec::new();
        let mut four_nb = Vec::new();
        let mut five_nb = Vec::new();
        let mut highest = CardType::One;
        let mut score = Self::Nothing;

        for card in cards {
            if let Some(v) = nb_of.get_mut(&card) {
                *v += 1;
            } else {
                nb_of.insert(card, 1);
            }

            highest = card.max(highest);
        }

        for (c, n) in nb_of {
            match n {
                2 => pair_nb.push(c),
                3 => three_nb.push(c),
                4 => four_nb.push(c),
                5 => five_nb.push(c),
                _ => (),
            }
        }

        score = if pair_nb.len() == 1 {
            score.max(Self::Pair(pair_nb[0]))
        } else if pair_nb.len() == 2 {
            score.max(Self::TwoPair(pair_nb[0], pair_nb[1]))
        } else if three_nb.len() == 1 {
            score.max(Self::ThreeOfAKind(three_nb[0]))
        } else if pair_nb.len() == 1 && three_nb.len() == 1 {
            score.max(Self::FullHouse(pair_nb[0], three_nb[0]))
        } else if four_nb.len() == 1 {
            score.max(Self::FourOfAKind(four_nb[0]))
        } else if five_nb.len() == 1 {
            score.max(Self::FiveOfAKind(five_nb[0]))
        } else {
            score.max(Self::Nothing)
        };

        score
    }
}

impl Day for Seven {
    fn one(testing: bool) -> String {
        let content = AdventOfCode::read_file_to_string(testing, "seven");
        let games = content.lines();
        let mut scores: Vec<(CardScore, u32)> = Vec::new();
        let mut r = 0;

        for game in games {
            let game = game.split(' ').collect::<Vec<_>>();

            scores.push((
                CardScore::get_score(
                    game[0]
                        .split("")
                        .skip(1)
                        .take_while(|&c| !c.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect(),
                ),
                game[1].parse().unwrap(),
            ));
        }

        scores.sort();

        for (i, (c, s)) in scores.iter().enumerate() {
            println!("{c:?} {r} {s} {}", i + 1);
            r += (i + 1) as u32 * *s;
        }

        // r < 254269559
        // r < 254720290

        r.to_string()
    }
}
