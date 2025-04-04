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
