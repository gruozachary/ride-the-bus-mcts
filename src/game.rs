use std::{fmt, str::FromStr};

use rand::seq::IndexedRandom;

use crate::card;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HiLo {
    Higher,
    Lower,
}
impl HiLo {
    fn is_true(&self, card0: &card::Card, card1: &card::Card) -> bool {
        match self {
            Self::Higher => card0.value > card1.value,
            Self::Lower => card0.value < card1.value,
        }
    }
}
impl fmt::Display for HiLo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Higher => write!(f, "Higher"),
            Self::Lower => write!(f, "Lower"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InOut {
    Inside,
    Outside,
}
impl InOut {
    fn is_true(&self, card0: &card::Card, card1: &card::Card, card2: &card::Card) -> bool {
        let small = Ord::min(card0.value, card2.value);
        let large = Ord::max(card0.value, card2.value);

        match self {
            Self::Inside => small < card1.value && card1.value < large,
            Self::Outside => card1.value < small || large < card1.value,
        }
    }
}
impl fmt::Display for InOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inside => write!(f, "Inside"),
            Self::Outside => write!(f, "Outside"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Start,
    Stage1PlayerPicked(card::Colour),
    Stage1DealerPicked(card::Colour, card::Card),
    Stage2PlayerPicked(card::Card, HiLo),
    Stage2DealerPicked(card::Card, HiLo, card::Card),
    Stage3PlayerPicked(card::Card, card::Card, InOut),
    Stage3DealerPicked(card::Card, card::Card, InOut, card::Card),
    Stage4PlayerPicked(card::Card, card::Card, card::Card, card::Suit),
    Finished(u32),
}
impl State {
    pub fn apply_move(&self, mov: Move) -> Option<Self> {
        match self {
            State::Start => {
                if let Move::Colour(colour) = mov {
                    Some(State::Stage1PlayerPicked(colour))
                } else {
                    None
                }
            }
            State::Stage1PlayerPicked(colour) => {
                if let Move::Card(card) = mov {
                    if colour != &card.suit.colour() {
                        Some(State::Finished(0))
                    } else {
                        Some(State::Stage1DealerPicked(*colour, card))
                    }
                } else {
                    None
                }
            }
            State::Stage1DealerPicked(_, card) => {
                if let Move::HiLo(hi_lo) = mov {
                    Some(State::Stage2PlayerPicked(*card, hi_lo))
                } else if let Move::Finish = mov {
                    Some(State::Finished(2))
                } else {
                    None
                }
            }
            State::Stage2PlayerPicked(card, hi_lo) => {
                if let Move::Card(card1) = mov {
                    if !hi_lo.is_true(&card1, card) {
                        Some(State::Finished(0))
                    } else {
                        Some(State::Stage2DealerPicked(*card, *hi_lo, card1))
                    }
                } else {
                    None
                }
            }
            State::Stage2DealerPicked(card, _, card1) => {
                if let Move::InOut(in_out) = mov {
                    Some(State::Stage3PlayerPicked(*card, *card1, in_out))
                } else if let Move::Finish = mov {
                    Some(State::Finished(3))
                } else {
                    None
                }
            }
            State::Stage3PlayerPicked(card, card1, in_out) => {
                if let Move::Card(card2) = mov {
                    if !in_out.is_true(card, &card2, card1) {
                        Some(State::Finished(0))
                    } else {
                        Some(State::Stage3DealerPicked(*card, *card1, *in_out, card2))
                    }
                } else {
                    None
                }
            }
            State::Stage3DealerPicked(card, card1, _, card2) => {
                if let Move::Suit(suit) = mov {
                    Some(State::Stage4PlayerPicked(*card, *card1, *card2, suit))
                } else if let Move::Finish = mov {
                    Some(State::Finished(4))
                } else {
                    None
                }
            }
            State::Stage4PlayerPicked(_, _, _, suit) => {
                if let Move::Card(card3) = mov {
                    if suit != &card3.suit {
                        Some(State::Finished(0))
                    } else {
                        Some(State::Finished(20))
                    }
                } else {
                    None
                }
            }
            State::Finished(_) => None,
        }
    }
    pub fn get_valid_moves(&self) -> Vec<Move> {
        match self {
            State::Start => vec![
                Move::Colour(card::Colour::Red),
                Move::Colour(card::Colour::Black),
            ],
            State::Stage1PlayerPicked(_) => card::Card::rest_of_deck(&[])
                .into_iter()
                .map(|c| Move::Card(c))
                .collect(),
            State::Stage1DealerPicked(_, _) => vec![
                Move::HiLo(HiLo::Higher),
                Move::HiLo(HiLo::Lower),
                Move::Finish,
            ],
            State::Stage2PlayerPicked(card, _) => card::Card::rest_of_deck(&[*card])
                .into_iter()
                .map(|c| Move::Card(c))
                .collect(),
            State::Stage2DealerPicked(_, _, _) => vec![
                Move::InOut(InOut::Inside),
                Move::InOut(InOut::Outside),
                Move::Finish,
            ],
            State::Stage3PlayerPicked(card, card1, _) => card::Card::rest_of_deck(&[*card, *card1])
                .into_iter()
                .map(|c| Move::Card(c))
                .collect(),
            State::Stage3DealerPicked(_, _, _, _) => vec![
                Move::Suit(card::Suit::Hearts),
                Move::Suit(card::Suit::Diamonds),
                Move::Suit(card::Suit::Clubs),
                Move::Suit(card::Suit::Spades),
                Move::Finish,
            ],
            State::Stage4PlayerPicked(card, card1, card2, _) => {
                card::Card::rest_of_deck(&[*card, *card1, *card2])
                    .into_iter()
                    .map(|c| Move::Card(c))
                    .collect()
            }
            State::Finished(_) => vec![],
        }
    }

    pub fn playout<R: rand::Rng>(&self, mut rng: &mut R) -> u32 {
        let mut game = *self;
        loop {
            if let State::Finished(x) = game {
                return x;
            }
            let valid_moves = game.get_valid_moves();
            let mov = valid_moves.choose(&mut rng).unwrap();
            game = game.apply_move(*mov).unwrap();
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Finished(_) => true,
            _ => false,
        }
    }

    pub fn is_dealer_turn(&self) -> bool {
        match self {
            State::Start => false,
            State::Stage1PlayerPicked(_) => true,
            State::Stage1DealerPicked(_, _) => false,
            State::Stage2PlayerPicked(_, _) => true,
            State::Stage2DealerPicked(_, _, _) => false,
            State::Stage3PlayerPicked(_, _, _) => true,
            State::Stage3DealerPicked(_, _, _, _) => false,
            State::Stage4PlayerPicked(_, _, _, _) => true,
            State::Finished(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Colour(card::Colour),
    HiLo(HiLo),
    InOut(InOut),
    Suit(card::Suit),
    Card(card::Card),
    Finish,
}
impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        match s.as_str() {
            "red" => Ok(Move::Colour(card::Colour::Red)),
            "black" => Ok(Move::Colour(card::Colour::Black)),
            "higher" => Ok(Move::HiLo(HiLo::Higher)),
            "lower" => Ok(Move::HiLo(HiLo::Lower)),
            "inside" => Ok(Move::InOut(InOut::Inside)),
            "outside" => Ok(Move::InOut(InOut::Outside)),
            "hearts" => Ok(Move::Suit(card::Suit::Hearts)),
            "diamonds" => Ok(Move::Suit(card::Suit::Diamonds)),
            "clubs" => Ok(Move::Suit(card::Suit::Clubs)),
            "spades" => Ok(Move::Suit(card::Suit::Spades)),
            "finish" => Ok(Move::Finish),
            _ => {
                let words: Vec<&str> = s.split_ascii_whitespace().collect();
                if words.len() != 3 || words[1] != "of" {
                    return Err("Parse failed");
                }
                let suit = match words[2] {
                    "hearts" => card::Suit::Hearts,
                    "diamonds" => card::Suit::Diamonds,
                    "clubs" => card::Suit::Clubs,
                    "spades" => card::Suit::Spades,
                    _ => return Err("Parse failed"),
                };
                let value = match words[0] {
                    "two" => card::Value::Two,
                    "three" => card::Value::Three,
                    "four" => card::Value::Four,
                    "five" => card::Value::Five,
                    "six" => card::Value::Six,
                    "seven" => card::Value::Seven,
                    "eight" => card::Value::Eight,
                    "nine" => card::Value::Nine,
                    "ten" => card::Value::Ten,
                    "jack" => card::Value::Jack,
                    "queen" => card::Value::Queen,
                    "king" => card::Value::King,
                    "ace" => card::Value::Ace,
                    _ => return Err("Parse failed"),
                };
                Ok(Move::Card(card::Card::new(suit, value)))
            }
        }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Colour(colour) => colour.fmt(f),
            Move::HiLo(hi_lo) => hi_lo.fmt(f),
            Move::InOut(in_out) => in_out.fmt(f),
            Move::Suit(suit) => suit.fmt(f),
            Move::Card(card) => card.fmt(f),
            Move::Finish => write!(f, "Finish"),
        }
    }
}
