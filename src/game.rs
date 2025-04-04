use crate::card;

#[derive(Clone, Copy)]
enum HiLo {
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

#[derive(Clone, Copy)]
enum InOut {
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

#[derive(Clone, Copy)]
enum State {
    Start,
    Stage1PlayerPicked(card::Colour),
    Stage1DealerPicked(card::Colour, card::Card),
    Stage2PlayerPicked(card::Card, HiLo),
    Stage2DealerPicked(card::Card, HiLo, card::Card),
    Stage3PlayerPicked(card::Card, card::Card, InOut),
    Stage3DealerPicked(card::Card, card::Card, InOut, card::Card),
    Stage4PlayerPicked(card::Card, card::Card, card::Card, card::Suit),
    Stage4DealerPicked(card::Card, card::Card, card::Card, card::Suit, card::Card),
    Finished(u32),
}
impl State {
    fn apply_move(&self, mov: Move) -> Option<Self> {
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
                    Some(State::Stage1DealerPicked(*colour, card))
                } else {
                    None
                }
            }
            State::Stage1DealerPicked(colour, card) => {
                if colour != &card.suit.colour() {
                    Some(State::Finished(0))
                } else if let Move::HiLo(hi_lo) = mov {
                    Some(State::Stage2PlayerPicked(*card, hi_lo))
                } else if let Move::Finish = mov {
                    Some(State::Finished(2))
                } else {
                    None
                }
            }
            State::Stage2PlayerPicked(card, hi_lo) => {
                if let Move::Card(card1) = mov {
                    Some(State::Stage2DealerPicked(*card, *hi_lo, card1))
                } else {
                    None
                }
            }
            State::Stage2DealerPicked(card, hi_lo, card1) => {
                if !hi_lo.is_true(card1, card) {
                    Some(State::Finished(0))
                } else if let Move::InOut(in_out) = mov {
                    Some(State::Stage3PlayerPicked(*card, *card1, in_out))
                } else if let Move::Finish = mov {
                    Some(State::Finished(6))
                } else {
                    None
                }
            }
            State::Stage3PlayerPicked(card, card1, in_out) => {
                if let Move::Card(card2) = mov {
                    Some(State::Stage3DealerPicked(*card, *card1, *in_out, card2))
                } else {
                    None
                }
            }
            State::Stage3DealerPicked(card, card1, in_out, card2) => {
                if !in_out.is_true(card, card2, card1) {
                    Some(State::Finished(0))
                } else if let Move::Suit(suit) = mov {
                    Some(State::Stage4PlayerPicked(*card, *card1, *card2, suit))
                } else if let Move::Finish = mov {
                    Some(State::Finished(24))
                } else {
                    None
                }
            }
            State::Stage4PlayerPicked(card, card1, card2, suit) => {
                if let Move::Card(card3) = mov {
                    Some(State::Stage4DealerPicked(
                        *card, *card1, *card2, *suit, card3,
                    ))
                } else {
                    None
                }
            }
            State::Stage4DealerPicked(card, card1, card2, suit, card3) => {
                if suit != &card3.suit {
                    Some(State::Finished(0))
                } else if let Move::Finish = mov {
                    Some(State::Finished(480))
                } else {
                    None
                }
            }
            State::Finished(_) => None,
        }
    }
}

enum Move {
    Colour(card::Colour),
    HiLo(HiLo),
    InOut(InOut),
    Suit(card::Suit),
    Card(card::Card),
    Finish,
}
